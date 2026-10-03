// SPDX-License-Identifier: GPL-3.0-or-later

//! The rule editor: one mail rule in a dialog over the window, opened from
//! Settings > Folders & rules (New rule, or a rule's pencil) and from a
//! mail's right-click menu (Make a rule…, filled in with its sender). A
//! name; "When a new mail matches all/any of these" with condition rows;
//! "Then" with action rows; Stop here; the accounts; and a light panel
//! counting the inbox mail of the last 30 days it matches, with "Also
//! apply to these". The daemon saves and runs rules
//! (`docs/ARCHITECTURE.md` §9.4); its reasons for refusing one show in
//! the dialog.

use std::time::Duration;

use gpui::{
    AnyElement, App, ClickEvent, Context, ElementId, Entity, Focusable, FontWeight, HighlightStyle,
    MouseButton, Pixels, Point, SharedString, Stateful, StyledText, Subscription, Task, Window,
    anchored, deferred, div, prelude::*, rgba,
};
use katna_core::{AccountId, MailCategory};
use katna_i18n::tr;
use katna_store::FolderId;
use katna_store::rules::{
    Action, Comparator, Condition, Field, MAX_READ_AFTER_DAYS, MatchMode, Rule, RunsNote, RunsOn,
};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;
use katna_ui::{InputEvent, TextArea, TextInput};

use super::MailWindow;
use super::add_account::text_button;
use crate::sidebar::Role;
use crate::theme::{Theme, fade};
use crate::widgets::{Check, FocusRing, checkbox, filled_button, icon, icon_button, menu, tip};
use crate::{daemon, data, format};

const WIDTH: f32 = 680.0;
/// How far back the panel counts matching mail, and "Also apply to
/// these" applies the rule.
pub(super) const PREVIEW_DAYS: u32 = 30;
/// The count waits this long after the last change.
const PREVIEW_DELAY: Duration = Duration::from_millis(400);
/// Where the dialog goes narrow: the condition and action rows wrap.
const NARROW: f32 = 520.0;

/// What a condition can look at, in the order the menu lists them.
const FIELDS: [Field; 11] = [
    Field::From,
    Field::To,
    Field::Cc,
    Field::AnyRecipient,
    Field::ReplyTo,
    Field::Subject,
    Field::Body,
    Field::AttachmentName,
    Field::HasAttachment,
    Field::MailingList,
    Field::Tab,
];

const COMPARATORS: [Comparator; 6] = [
    Comparator::Contains,
    Comparator::NotContains,
    Comparator::BeginsWith,
    Comparator::EndsWith,
    Comparator::Equals,
    Comparator::Matches,
];

/// What an action row does, before its folder, address or days.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionKind {
    Move,
    Archive,
    Trash,
    MarkRead,
    Star,
    MarkImportant,
    AddLabel,
    Forward,
    DontNotify,
    MarkReadAfter,
}

impl ActionKind {
    const ALL: [Self; 10] = [
        Self::Move,
        Self::Archive,
        Self::Trash,
        Self::AddLabel,
        Self::MarkRead,
        Self::Star,
        Self::MarkImportant,
        Self::Forward,
        Self::DontNotify,
        Self::MarkReadAfter,
    ];

    fn of(action: &Action) -> Self {
        match action {
            Action::Move { .. } => Self::Move,
            Action::Archive => Self::Archive,
            Action::Trash => Self::Trash,
            Action::MarkRead => Self::MarkRead,
            Action::Star => Self::Star,
            Action::MarkImportant => Self::MarkImportant,
            Action::AddLabel { .. } => Self::AddLabel,
            Action::Forward { .. } => Self::Forward,
            Action::DontNotify => Self::DontNotify,
            Action::MarkReadAfter { .. } => Self::MarkReadAfter,
        }
    }

    fn label(self) -> String {
        match self {
            Self::Move => tr!("rules-action-move"),
            Self::Archive => tr!("rules-action-archive"),
            Self::Trash => tr!("rules-action-trash"),
            Self::MarkRead => tr!("rules-action-mark-read"),
            Self::Star => tr!("rules-action-star"),
            Self::MarkImportant => tr!("rules-action-important"),
            Self::AddLabel => tr!("rules-action-label"),
            Self::Forward => tr!("rules-action-forward"),
            Self::DontNotify => tr!("rules-action-dont-notify"),
            Self::MarkReadAfter => tr!("rules-action-read-after"),
        }
    }

    fn takes_text(self) -> bool {
        matches!(self, Self::Forward | Self::MarkReadAfter)
    }
}

/// The name of what a condition looks at, as its menu shows it.
pub(super) fn field_label(field: Field) -> String {
    match field {
        Field::From => tr!("rules-field-from"),
        Field::To => tr!("rules-field-to"),
        Field::Cc => tr!("rules-field-cc"),
        Field::AnyRecipient => tr!("rules-field-any-recipient"),
        Field::ReplyTo => tr!("rules-field-reply-to"),
        Field::Subject => tr!("rules-field-subject"),
        Field::Body => tr!("rules-field-body"),
        Field::AttachmentName => tr!("rules-field-attachment-name"),
        Field::HasAttachment => tr!("rules-field-has-attachment"),
        Field::MailingList => tr!("rules-field-mailing-list"),
        Field::Tab => tr!("rules-field-tab"),
    }
}

/// An inbox tab's name, as the mail list's tabs show it.
pub(super) fn tab_label(tab: MailCategory) -> String {
    match tab {
        MailCategory::Primary => tr!("tab-primary"),
        MailCategory::Promotions => tr!("tab-promotions"),
        MailCategory::Social => tr!("tab-social"),
        MailCategory::Updates => tr!("tab-updates"),
        MailCategory::Forums => tr!("tab-forums"),
    }
}

/// The tab an "Inbox tab" condition names (Primary for one it can't read).
fn tab_of(value: &str) -> MailCategory {
    value.trim().parse().unwrap_or_default()
}

/// How many alternatives of a pattern a rule's line names before "and N
/// more".
const MAX_LISTED: usize = 3;

/// A pattern that is only words or addresses joined by `|`
/// ("irctc.co.in|railyatri.in"), as its alternatives; `None` for any
/// other pattern.
fn plain_alternatives(pattern: &str) -> Option<Vec<String>> {
    let plain = |c: char| c.is_alphanumeric() || matches!(c, '.' | '@' | '-' | ' ' | '_');
    let words: Vec<String> = pattern.split('|').map(|w| w.trim().to_owned()).collect();
    words
        .iter()
        .all(|w| !w.is_empty() && w.chars().all(plain))
        .then_some(words)
}

pub(super) fn comparator_label(comparator: Comparator) -> String {
    match comparator {
        Comparator::Contains => tr!("rules-comparator-contains"),
        Comparator::NotContains => tr!("rules-comparator-not-contains"),
        Comparator::BeginsWith => tr!("rules-comparator-begins-with"),
        Comparator::EndsWith => tr!("rules-comparator-ends-with"),
        Comparator::Equals => tr!("rules-comparator-equals"),
        Comparator::Matches => tr!("rules-comparator-matches"),
    }
}

/// Whether a yes-or-no condition's value means no.
fn means_no(value: &str) -> bool {
    katna_store::rules::says_no(value)
}

/// A rule in one line, for its row in Settings: "From contains
/// substack.com → skip the inbox, label Reading". `folder` names a folder
/// by its ID, or `None` for one that is gone.
pub(super) fn summary(rule: &Rule, folder: impl Fn(i64) -> Option<String>) -> String {
    // "A, B or C" (or "and", as the rule matches).
    let join = |items: Vec<String>| -> Option<String> {
        let mut items = items.into_iter();
        let mut text = items.next()?;
        let rest: Vec<String> = items.collect();
        let last = rest.len().saturating_sub(1);
        for (ix, next) in rest.into_iter().enumerate() {
            text = match rule.match_mode {
                _ if ix < last => tr!("rules-summary-list", first = text, next = next),
                MatchMode::All => tr!("rules-summary-and", first = text, next = next),
                MatchMode::Any => tr!("rules-summary-or", first = text, next = next),
            };
        }
        Some(text)
    };
    // Conditions on the same field that compare the same way say it
    // once: "From contains substack.com or medium.com".
    // Alternatives inside one condition always read "or".
    let any = |mut values: Vec<String>| -> String {
        // A long list names its first few and counts the rest.
        if values.len() > MAX_LISTED + 1 {
            let more = values.len() - MAX_LISTED;
            values.truncate(MAX_LISTED);
            values.push(tr!("rules-summary-more", count = more));
        }
        let mut items = values.into_iter();
        let mut text = items.next().unwrap_or_default();
        let rest: Vec<String> = items.collect();
        let last = rest.len().saturating_sub(1);
        for (ix, next) in rest.into_iter().enumerate() {
            text = if ix < last {
                tr!("rules-summary-list", first = text, next = next)
            } else {
                tr!("rules-summary-or", first = text, next = next)
            };
        }
        text
    };
    // A pattern of plain words joined by `|` reads as "contains" them.
    let mut groups: Vec<(Field, Comparator, Vec<String>)> = Vec::new();
    for c in &rule.conditions {
        let (comparator, value) = match c.comparator {
            Comparator::Matches if !c.field.is_yes_no() && c.field != Field::Tab => {
                match plain_alternatives(&c.value) {
                    Some(words) => (Comparator::Contains, any(words)),
                    None => (c.comparator, c.value.trim().to_owned()),
                }
            }
            comparator => (comparator, c.value.trim().to_owned()),
        };
        match groups.last_mut() {
            Some((field, last, values))
                if *field == c.field
                    && *last == comparator
                    && !c.field.is_yes_no()
                    && c.field != Field::Tab =>
            {
                values.push(value)
            }
            _ => groups.push((c.field, comparator, vec![value])),
        }
    }
    let conditions = groups
        .into_iter()
        .map(|(field, comparator, values)| match field {
            Field::HasAttachment if values.iter().all(|v| means_no(v)) => {
                tr!("rules-summary-no-attachment")
            }
            Field::HasAttachment => tr!("rules-summary-has-attachment"),
            Field::MailingList if values.iter().all(|v| means_no(v)) => {
                tr!("rules-summary-not-mailing-list")
            }
            Field::MailingList => tr!("rules-summary-mailing-list"),
            Field::Tab => {
                let tab = tab_label(tab_of(&values.concat()));
                if comparator == Comparator::NotContains {
                    tr!("rules-summary-not-tab", tab = tab)
                } else {
                    tr!("rules-summary-tab", tab = tab)
                }
            }
            field => tr!(
                "rules-summary-condition",
                field = field_label(field),
                comparator = comparator_label(comparator),
                value = join(values).unwrap_or_default()
            ),
        })
        .collect();
    let when = join(conditions);
    // A folder in each account says its name once.
    let mut said: Vec<String> = Vec::new();
    for action in &rule.actions {
        let text = action_text(action, &folder);
        if !said.contains(&text) {
            said.push(text);
        }
    }
    let then = said
        .into_iter()
        .reduce(|first, next| tr!("rules-summary-list", first = first, next = next));
    match (when, then) {
        (Some(when), Some(then)) => tr!("rules-summary", when = when, then = then),
        (when, then) => when.or(then).unwrap_or_default(),
    }
}

/// One action as a rule's summary says it: "move to Receipts".
fn action_text(action: &Action, folder: &impl Fn(i64) -> Option<String>) -> String {
    let named = |id: i64| folder(id).unwrap_or_else(|| tr!("rules-summary-folder-gone"));
    match action {
        Action::Move { folder } => tr!("rules-summary-move", folder = named(*folder)),
        Action::Archive => tr!("rules-summary-archive"),
        Action::Trash => tr!("rules-summary-trash"),
        Action::MarkRead => tr!("rules-summary-mark-read"),
        Action::Star => tr!("rules-summary-star"),
        Action::MarkImportant => tr!("rules-summary-important"),
        Action::AddLabel { folder } => tr!("rules-summary-label", label = named(*folder)),
        Action::Forward { to } => tr!("rules-summary-forward", address = to.trim()),
        Action::DontNotify => tr!("rules-summary-dont-notify"),
        Action::MarkReadAfter { days } => tr!("rules-summary-read-after", count = *days),
    }
}

/// The line under the editor's preview: where the rule runs, and why it
/// runs in Katna when the account's mail service runs rules.
pub(super) fn runs_text(rule: &Rule, folder: impl Fn(i64) -> Option<String>) -> String {
    match (rule.runs_on, &rule.runs_note) {
        (RunsOn::Gmail, _) => tr!("rules-editor-runs-gmail"),
        (RunsOn::Sieve, _) => tr!("rules-editor-runs-sieve"),
        (RunsOn::Katna, Some(note)) => note_text(note, &folder),
        (RunsOn::Katna, None) => tr!("rules-editor-runs-katna"),
    }
}

/// Why a rule runs in Katna: "Runs in Katna: Gmail filters can't do
/// “don't notify”."
pub(super) fn note_text(note: &RunsNote, folder: &impl Fn(i64) -> Option<String>) -> String {
    let gmail = |service: &RunsOn| *service == RunsOn::Gmail;
    match note {
        RunsNote::Action { service, action } => {
            let action = action_text(action, folder);
            if gmail(service) {
                tr!("rules-note-gmail-action", action = action)
            } else {
                tr!("rules-note-sieve-action", action = action)
            }
        }
        RunsNote::Condition {
            service,
            field,
            comparator,
        } => {
            let test = if field.is_yes_no() || *field == Field::Tab {
                field_label(*field)
            } else {
                tr!(
                    "rules-note-test",
                    field = field_label(*field),
                    comparator = comparator_label(*comparator)
                )
            };
            if gmail(service) {
                tr!("rules-note-gmail-condition", test = test)
            } else {
                tr!("rules-note-sieve-condition", test = test)
            }
        }
        RunsNote::Order { .. } => tr!("rules-note-order"),
        RunsNote::Stop { .. } => tr!("rules-note-gmail-stop"),
        RunsNote::ForwardAddress { to } => tr!("rules-note-gmail-forward", address = to.as_str()),
        RunsNote::Folder { service } if gmail(service) => tr!("rules-note-gmail-folder"),
        RunsNote::Folder { .. } => tr!("rules-note-sieve-folder"),
        RunsNote::SignIn => tr!("rules-note-gmail-sign-in"),
        RunsNote::OtherScript { name } => {
            tr!("rules-note-sieve-other-script", name = name.as_str())
        }
        RunsNote::Failed { service, error } => {
            let error = format::sentence(error);
            let error = error.trim_end_matches('.');
            if gmail(service) {
                tr!("rules-note-gmail-failed", error = error)
            } else {
                tr!("rules-note-sieve-failed", error = error)
            }
        }
    }
}

/// Why the daemon switched a rule off, in words for its row. The daemon
/// gives its reasons in English (`katna_sync::rules`).
pub(super) fn error_text(error: &str) -> String {
    if error.starts_with("folder ") && error.ends_with("no longer exists") {
        tr!("rules-error-folder-gone")
    } else if error.contains("no archive folder") {
        tr!("rules-error-no-archive")
    } else if error.contains("no Trash folder") {
        tr!("rules-error-no-trash")
    } else if error.contains("cannot send") {
        tr!("rules-error-cannot-send")
    } else {
        let sentence = format::sentence(error);
        tr!("rules-error-other", error = sentence.trim_end_matches('.'))
    }
}

/// Where a rule runs, for the tag on its row.
pub(super) fn runs_on_label(runs_on: RunsOn) -> String {
    match runs_on {
        RunsOn::Katna => tr!("rules-runs-katna"),
        RunsOn::Gmail => tr!("rules-runs-gmail"),
        RunsOn::Sieve => tr!("rules-runs-sieve"),
    }
}

/// A search for about the mail `rule` matches over the last `days` days,
/// for "Show them": `None` when a search can't say what the rule does
/// (other comparators, the text, Reply-to).
pub(super) fn search_query(rule: &Rule, days: u32) -> Option<String> {
    let parts = rule
        .conditions
        .iter()
        .map(|c| {
            if c.field == Field::HasAttachment {
                let has = "has:attachment";
                return Some(if means_no(&c.value) {
                    format!("-{has}")
                } else {
                    has.to_owned()
                });
            }
            let operator = match c.field {
                Field::From => "from",
                // The search's `to:` is To, Cc or Bcc, like Gmail's.
                Field::To | Field::AnyRecipient => "to",
                Field::Cc => "cc",
                Field::Subject => "subject",
                Field::AttachmentName => "filename",
                _ => return None,
            };
            let not = match c.comparator {
                Comparator::Contains => "",
                Comparator::NotContains => "-",
                _ => return None,
            };
            let value: String = c.value.trim().chars().filter(|c| *c != '"').collect();
            if value.is_empty() {
                return None;
            }
            Some(format!("{not}{operator}:\"{value}\""))
        })
        .collect::<Option<Vec<_>>>()?;
    if parts.is_empty() {
        return None;
    }
    let matched = match rule.match_mode {
        MatchMode::All => parts.join(" "),
        MatchMode::Any if parts.len() > 1 => format!("({})", parts.join(" OR ")),
        MatchMode::Any => parts.join(" "),
    };
    Some(format!("{matched} in:inbox newer_than:{days}d"))
}

/// A menu of the editor, open at a point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pick {
    Mode,
    Field(usize),
    Comparator(usize),
    Has(usize),
    Tab(usize),
    Action(usize),
    Folder(usize),
    Accounts,
}

/// The count in the light panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Preview {
    /// Nothing to count yet: no condition or no account.
    Nothing,
    Counting,
    Count(u32),
}

struct ConditionRow {
    field: Field,
    comparator: Comparator,
    value: Entity<TextArea>,
    /// A yes-or-no condition's choice.
    has: bool,
    /// An "Inbox tab" condition's tab.
    tab: MailCategory,
    _subscription: Subscription,
}

struct ActionRow {
    kind: ActionKind,
    /// The folder or label of Move to and Add label, once chosen.
    folder: Option<i64>,
    /// Forward's address, or Mark read after's days.
    text: Entity<TextInput>,
    _subscription: Subscription,
}

pub(super) struct RuleEditor {
    /// The rule as it was opened: its ID (0 for a new one), place, on or
    /// off and where it runs, which the editor keeps.
    rule: Rule,
    name: Entity<TextInput>,
    _name: Subscription,
    mode: MatchMode,
    conditions: Vec<ConditionRow>,
    actions: Vec<ActionRow>,
    stop: bool,
    accounts: Vec<i64>,
    also_apply: bool,
    preview: Preview,
    preview_task: Option<Task<()>>,
    pick: Option<(Pick, Point<Pixels>)>,
    error: Option<String>,
    busy: bool,
    asking_delete: bool,
    closing: bool,
    shown: Spring,
    /// Folders saving makes first, as (stand-in ID below 0, account,
    /// name): a starter rule's folder an account doesn't have yet.
    new_folders: Vec<(i64, i64, String)>,
}

impl RuleEditor {
    /// The rule as the editor has it. `strict` refuses one that can't be
    /// saved yet (a folder not chosen, days that aren't a number); without
    /// it, for counting, conditions without a value are left out.
    fn build(&self, strict: bool, cx: &App) -> Result<Rule, String> {
        let conditions = self
            .conditions
            .iter()
            .filter_map(|row| {
                let picked = row.field.is_yes_no() || row.field == Field::Tab;
                let value = if row.field.is_yes_no() {
                    if row.has { "true" } else { "false" }.to_owned()
                } else if row.field == Field::Tab {
                    row.tab.as_str().to_owned()
                } else {
                    row.value.read(cx).text().trim().to_owned()
                };
                // A tab is "is in" unless it says "isn't".
                let comparator = match (row.field, row.comparator) {
                    (Field::Tab, Comparator::NotContains) => Comparator::NotContains,
                    (Field::Tab, _) => Comparator::Equals,
                    (_, comparator) => comparator,
                };
                (strict || picked || !value.is_empty()).then_some(Condition {
                    field: row.field,
                    comparator,
                    value,
                })
            })
            .collect();
        let mut actions = Vec::new();
        for row in &self.actions {
            let text = row.text.read(cx).text().trim().to_owned();
            // A folder still to make counts as chosen only for saving.
            let folder = row.folder.filter(|f| *f > 0 || (strict && *f < 0));
            actions.push(match row.kind {
                ActionKind::Move | ActionKind::AddLabel if strict && folder.is_none() => {
                    return Err(tr!("rules-editor-needs-folder"));
                }
                ActionKind::Move => Action::Move {
                    folder: folder.unwrap_or(0),
                },
                ActionKind::AddLabel => Action::AddLabel {
                    folder: folder.unwrap_or(0),
                },
                ActionKind::Archive => Action::Archive,
                ActionKind::Trash => Action::Trash,
                ActionKind::MarkRead => Action::MarkRead,
                ActionKind::Star => Action::Star,
                ActionKind::MarkImportant => Action::MarkImportant,
                ActionKind::Forward => Action::Forward { to: text },
                ActionKind::DontNotify => Action::DontNotify,
                ActionKind::MarkReadAfter => match text.parse::<u32>() {
                    Ok(days) if (1..=MAX_READ_AFTER_DAYS).contains(&days) => {
                        Action::MarkReadAfter { days }
                    }
                    _ if strict => return Err(tr!("rules-editor-needs-days")),
                    _ => Action::MarkReadAfter { days: 1 },
                },
            });
        }
        Ok(Rule {
            name: self.name.read(cx).text().trim().to_owned(),
            match_mode: self.mode,
            conditions,
            actions,
            stop: self.stop,
            accounts: self.accounts.clone(),
            last_error: None,
            ..self.rule.clone()
        })
    }
}

impl MailWindow {
    /// Opens the editor on `rule`: a saved one, or a new one (ID 0) to
    /// fill in. A Move to or Add label with folder 0 has none chosen yet;
    /// one below 0 is a folder of `new_folders` that saving makes.
    pub(super) fn open_rule_editor(
        &mut self,
        new_folders: Vec<(i64, i64, String)>,
        rule: Rule,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_context_menu(cx);
        let name = self.rule_input(tr!("rules-editor-name-hint"), rule.name.clone(), window, cx);
        let conditions = rule
            .conditions
            .iter()
            .map(|c| self.condition_row(c, window, cx))
            .collect();
        let actions = rule
            .actions
            .iter()
            .map(|a| self.action_row(a, window, cx))
            .collect();
        window.focus(&name.0.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.rule_editor = Some(RuleEditor {
            mode: rule.match_mode,
            stop: rule.stop,
            accounts: rule.accounts.clone(),
            rule,
            name: name.0,
            _name: name.1,
            conditions,
            actions,
            also_apply: true,
            preview: Preview::Nothing,
            preview_task: None,
            pick: None,
            error: None,
            busy: false,
            asking_delete: false,
            closing: false,
            shown,
            new_folders,
        });
        self.count_rule_mail(cx);
        cx.notify();
    }

    /// The editor on a new rule for the mail accounts `accounts`.
    pub(super) fn new_rule(
        &mut self,
        accounts: Vec<i64>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rule = Rule {
            conditions: vec![Condition {
                field: Field::From,
                comparator: Comparator::Contains,
                value: String::new(),
            }],
            actions: vec![Action::Move { folder: 0 }],
            accounts,
            ..Rule::default()
        };
        self.open_rule_editor(Vec::new(), rule, window, cx);
    }

    /// Make a rule… on a mail: the editor filled in with its sender. (Move
    /// to's "Always move mail from … here" makes such a rule in one click:
    /// `folder_pick`.)
    pub(super) fn make_rule_from(
        &mut self,
        name: String,
        address: String,
        account: AccountId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rule = Rule {
            name,
            conditions: vec![Condition {
                field: Field::From,
                comparator: Comparator::Contains,
                value: address,
            }],
            actions: vec![Action::Move { folder: 0 }],
            accounts: vec![account.0],
            ..Rule::default()
        };
        self.open_rule_editor(Vec::new(), rule, window, cx);
    }

    /// Escape: an open menu of the editor, else the editor.
    pub(super) fn dismiss_rule_editor(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(e) = &mut self.rule_editor else {
            return false;
        };
        if e.pick.take().is_none() {
            self.close_rule_editor(cx);
        }
        true
    }

    fn close_rule_editor(&mut self, cx: &mut Context<Self>) {
        if let Some(e) = &mut self.rule_editor {
            e.closing = true;
            e.pick = None;
            e.preview_task = None;
            e.shown.set(0.0);
        }
        cx.notify();
    }

    /// A field of the editor; any change counts the matching mail again.
    fn rule_input(
        &mut self,
        placeholder: String,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextInput>, Subscription) {
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(placeholder, cx);
            input.set_text(text, cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &input,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Changed => this.rule_changed(cx),
                    InputEvent::Submit => this.save_rule_edit(cx),
                    InputEvent::Cancel => this.close_rule_editor(cx),
                },
            );
        (input, subscription)
    }

    /// A long field of the editor that wraps instead of scrolling
    /// sideways; any change counts the matching mail again.
    fn rule_area(
        &mut self,
        placeholder: String,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextArea>, Subscription) {
        let accent = rgba(self.theme(window).accent).into();
        let area = cx.new(|cx| {
            let mut area = TextArea::new(placeholder, cx);
            area.set_single_line(true);
            area.set_text(text, 0, cx);
            area.set_accent(accent);
            area
        });
        let subscription =
            cx.subscribe_in(
                &area,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Changed => this.rule_changed(cx),
                    InputEvent::Submit => this.save_rule_edit(cx),
                    InputEvent::Cancel => this.close_rule_editor(cx),
                },
            );
        (area, subscription)
    }

    fn condition_row(
        &mut self,
        condition: &Condition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ConditionRow {
        let has = condition.field.is_yes_no();
        let tab = condition.field == Field::Tab;
        let (value, subscription) = self.rule_area(
            tr!("rules-editor-value-hint"),
            if has || tab {
                String::new()
            } else {
                condition.value.clone()
            },
            window,
            cx,
        );
        ConditionRow {
            field: condition.field,
            comparator: condition.comparator,
            value,
            has: !has || !means_no(&condition.value),
            tab: if tab {
                tab_of(&condition.value)
            } else {
                MailCategory::Promotions
            },
            _subscription: subscription,
        }
    }

    fn action_row(
        &mut self,
        action: &Action,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ActionRow {
        let text = match action {
            Action::Forward { to } => to.clone(),
            Action::MarkReadAfter { days } => days.to_string(),
            _ => String::new(),
        };
        let kind = ActionKind::of(action);
        let (text, subscription) = self.rule_input(action_hint(kind), text, window, cx);
        ActionRow {
            kind,
            folder: action.folder().filter(|f| *f != 0),
            text,
            _subscription: subscription,
        }
    }

    /// Something in the editor changed.
    fn rule_changed(&mut self, cx: &mut Context<Self>) {
        if let Some(e) = &mut self.rule_editor {
            e.error = None;
        }
        self.count_rule_mail(cx);
        cx.notify();
    }

    /// Counts the mail the rule as it stands matches, a moment after the
    /// last change.
    fn count_rule_mail(&mut self, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        let Some(e) = &mut self.rule_editor else {
            return;
        };
        let rule = match e.build(false, cx) {
            Ok(rule) if !rule.conditions.is_empty() && !rule.accounts.is_empty() => rule,
            _ => {
                e.preview = Preview::Nothing;
                e.preview_task = None;
                return;
            }
        };
        if e.preview == Preview::Nothing {
            e.preview = Preview::Counting;
        }
        e.preview_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(PREVIEW_DELAY).await;
            let now = jiff::Timestamp::now().as_second();
            let count = cx
                .background_executor()
                .spawn(async move { data::rule_preview(&paths, &rule, PREVIEW_DAYS, now) })
                .await;
            this.update(cx, |this, cx| {
                if let Some(e) = &mut this.rule_editor {
                    e.preview = match count {
                        Ok(count) => Preview::Count(count),
                        Err(err) => {
                            tracing::warn!("{err}");
                            Preview::Nothing
                        }
                    };
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn edit_rule(&mut self, edit: impl FnOnce(&mut RuleEditor), cx: &mut Context<Self>) {
        if let Some(e) = &mut self.rule_editor {
            edit(e);
            e.pick = None;
        }
        self.rule_changed(cx);
    }

    fn add_rule_condition(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let row = self.condition_row(
            &Condition {
                field: Field::Subject,
                comparator: Comparator::Contains,
                value: String::new(),
            },
            window,
            cx,
        );
        let focus = row.value.focus_handle(cx);
        self.edit_rule(|e| e.conditions.push(row), cx);
        window.focus(&focus, cx);
    }

    fn add_rule_action(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // The first action not there yet.
        let taken: Vec<ActionKind> = self
            .rule_editor
            .as_ref()
            .map(|e| e.actions.iter().map(|a| a.kind).collect())
            .unwrap_or_default();
        let kind = [
            ActionKind::MarkRead,
            ActionKind::Star,
            ActionKind::MarkImportant,
            ActionKind::DontNotify,
            ActionKind::AddLabel,
        ]
        .into_iter()
        .find(|k| !taken.contains(k))
        .unwrap_or(ActionKind::MarkRead);
        let row = self.action_row(&action_of(kind, None, ""), window, cx);
        self.edit_rule(|e| e.actions.push(row), cx);
    }

    /// Action row `ix` does `kind` now.
    fn set_rule_action(
        &mut self,
        ix: usize,
        kind: ActionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row) = self
            .rule_editor
            .as_mut()
            .and_then(|e| e.actions.get_mut(ix))
        else {
            return;
        };
        if row.kind == kind {
            self.edit_rule(|_| {}, cx);
            return;
        }
        row.kind = kind;
        row.folder = None;
        let text = row.text.clone();
        text.update(cx, |input, cx| {
            input.set_placeholder(action_hint(kind));
            input.set_text(
                if kind == ActionKind::MarkReadAfter {
                    "1"
                } else {
                    ""
                },
                cx,
            );
        });
        // Only one action may move the mail, the others that do give
        // way, but a rule for several accounts may move to a folder in
        // each.
        let moves = |k: ActionKind| {
            matches!(
                k,
                ActionKind::Move | ActionKind::Archive | ActionKind::Trash
            )
        };
        if moves(kind)
            && let Some(e) = &mut self.rule_editor
        {
            let mut at = 0;
            e.actions.retain(|a| {
                let keep = at == ix
                    || !moves(a.kind)
                    || (kind == ActionKind::Move && a.kind == ActionKind::Move);
                at += 1;
                keep
            });
        }
        if kind.takes_text() {
            window.focus(&text.focus_handle(cx), cx);
        }
        self.edit_rule(|_| {}, cx);
    }

    fn open_rule_pick(&mut self, pick: Pick, at: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(e) = &mut self.rule_editor {
            e.pick = if e.pick.map(|p| p.0) == Some(pick) {
                None
            } else {
                Some((pick, at))
            };
        }
        cx.notify();
    }

    /// Saves the rule; then, when asked, runs it over the mail it matches.
    fn save_rule_edit(&mut self, cx: &mut Context<Self>) {
        let Some(e) = &mut self.rule_editor else {
            return;
        };
        if e.busy || e.closing {
            return;
        }
        let rule = match e.build(true, cx) {
            Ok(rule) => rule,
            Err(err) => {
                e.error = Some(err);
                cx.notify();
                return;
            }
        };
        let apply = e.also_apply && matches!(e.preview, Preview::Count(n) if n > 0);
        let make: Vec<(i64, i64, String)> = e
            .new_folders
            .iter()
            .filter(|(id, _, _)| rule.actions.iter().any(|a| a.folder() == Some(*id)))
            .cloned()
            .collect();
        let mut rule = rule;
        e.busy = true;
        e.error = None;
        e.pick = None;
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    for (stand_in, account, name) in make {
                        let id = daemon::create_folder(&connection, account, &name, None).await?;
                        for action in &mut rule.actions {
                            match action {
                                Action::Move { folder } | Action::AddLabel { folder }
                                    if *folder == stand_in =>
                                {
                                    *folder = id;
                                }
                                _ => {}
                            }
                        }
                    }
                    let id = daemon::rules::save(&connection, &rule).await?;
                    let applied = if apply {
                        Some(daemon::rules::apply(&connection, id, PREVIEW_DAYS).await)
                    } else {
                        None
                    };
                    Ok::<_, String>(applied)
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(applied) => {
                    if let Some(e) = &mut this.rule_editor {
                        e.busy = false;
                    }
                    this.close_rule_editor(cx);
                    let text = match applied {
                        None => tr!("rules-saved"),
                        Some(Ok(count)) => tr!("rules-saved-applied", count = count),
                        Some(Err(err)) => tr!("rules-apply-failed", error = err),
                    };
                    this.show_snackbar(text, None, cx);
                    this.load_rules(cx);
                }
                Err(err) => {
                    if let Some(e) = &mut this.rule_editor {
                        e.busy = false;
                        e.error = Some(format::sentence(&err));
                    }
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn delete_rule_edit(&mut self, cx: &mut Context<Self>) {
        let Some(e) = &self.rule_editor else {
            return;
        };
        let id = e.rule.id;
        self.close_rule_editor(cx);
        if id == 0 {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::rules::delete(&connection, id).await
                })
                .await;
            this.update(cx, |this, cx| {
                let text = match result {
                    Ok(()) => tr!("rules-deleted"),
                    Err(err) => tr!("rules-delete-failed", error = err),
                };
                this.show_snackbar(text, None, cx);
                this.load_rules(cx);
            })
            .ok();
        })
        .detach();
    }

    /// "Show them": searches the mail for what the rule matches.
    fn show_rule_mail(&mut self, query: String, window: &mut Window, cx: &mut Context<Self>) {
        self.close_rule_editor(cx);
        if self.settings_in_main() {
            self.close_settings_page(window, cx);
        }
        self.search_for(query, window, cx);
    }

    /// The name a folder shows as in the editor and in a rule's line.
    pub(super) fn rule_folder_name(&self, id: i64) -> Option<String> {
        let node = self.tree.node(FolderId(id))?;
        Some(if node.role == Role::Other {
            node.path.clone()
        } else {
            node.label()
        })
    }

    /// The folders an action row can choose from, with their names: any
    /// folder of the rule's accounts for Move to, their own folders and
    /// labels for Add label, and folders still to make. Only the account
    /// that has `chosen`, when a row has one: a row moves mail of one
    /// account.
    fn rule_folders_of(
        &self,
        e: &RuleEditor,
        kind: ActionKind,
        chosen: Option<i64>,
    ) -> Vec<(i64, String)> {
        let several = e.accounts.len() > 1;
        let owner = chosen.and_then(|f| {
            e.accounts.iter().copied().find(|&a| {
                e.new_folders
                    .iter()
                    .any(|(id, acc, _)| *id == f && *acc == a)
                    || self
                        .tree
                        .folders_of(AccountId(a))
                        .into_iter()
                        .any(|(id, _, _)| id.0 == f)
            })
        });
        let mut out = Vec::new();
        for &account in &e.accounts {
            if owner.is_some_and(|o| o != account) {
                continue;
            }
            let id = AccountId(account);
            let address = self.account_address(id).unwrap_or_default();
            let folders: Vec<i64> = if kind == ActionKind::AddLabel && self.tree.is_gmail(id) {
                // Labels are Gmail's (`SetLabels`); elsewhere a label is a
                // copy in the folder.
                self.tree
                    .nest_targets(id)
                    .into_iter()
                    .map(|(f, _)| f.0)
                    .collect()
            } else {
                self.tree
                    .folders_of(id)
                    .into_iter()
                    .filter(|(_, _, role)| {
                        !matches!(
                            role,
                            Role::Inbox | Role::Drafts | Role::Sent | Role::Snoozed | Role::Flagged
                        )
                    })
                    // A label keeps mail where it is: never in Trash or
                    // the archive.
                    .filter(|(_, _, role)| {
                        kind != ActionKind::AddLabel
                            || !matches!(role, Role::Archive | Role::Trash | Role::Junk)
                    })
                    .map(|(f, _, _)| f.0)
                    .collect()
            };
            let new = e
                .new_folders
                .iter()
                .filter(|(_, a, _)| *a == account)
                .map(|(f, _, name)| (*f, tr!("rules-editor-new-folder", name = name.as_str())));
            let named = folders
                .into_iter()
                .filter_map(|f| Some((f, self.rule_folder_name(f)?)))
                .chain(new)
                .collect::<Vec<_>>();
            for (folder, name) in named {
                out.push((
                    folder,
                    if several {
                        tr!(
                            "rules-editor-folder-of",
                            folder = name,
                            account = address.as_str()
                        )
                    } else {
                        name
                    },
                ));
            }
        }
        out
    }

    pub(super) fn render_rule_editor(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let e = self.rule_editor.as_mut()?;
        let t = e.shown.tick(window, reduce);
        if e.closing && e.shown.settled() {
            self.rule_editor = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let width = WIDTH.min(vw - 32.0);
        let narrow = width < NARROW;
        let e = self.rule_editor.as_ref()?;
        let body = self.rule_editor_body(e, narrow, th, cx);
        let pick = self.render_rule_pick(e, th, cx);
        let card = div()
            .id("rule-editor")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(width))
            .max_h(px((vh - 48.0).max(240.0)))
            // Shrinks to the room around it; the body scrolls.
            .min_h_0()
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                // Even margins inside the room below the top bar.
                .p(px(24.0))
                .child(
                    div()
                        .id("rule-editor-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_rule_editor(cx))),
                )
                .child(
                    div()
                        .max_h_full()
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .children(pick)
                .into_any_element(),
        )
    }

    fn rule_editor_body(
        &self,
        e: &RuleEditor,
        narrow: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let picked = e.pick.map(|p| p.0);
        let title = if e.rule.id == 0 {
            tr!("rules-editor-new-title")
        } else {
            tr!("rules-editor-edit-title")
        };
        let name = text_field("rule-name", &e.name, 44.0, th, cx);
        let when = div()
            .mt(px(16.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .text_size(px(14.0))
            .child(tr!("rules-editor-when"))
            .child(
                select_box(
                    "rule-mode",
                    match e.mode {
                        MatchMode::All => tr!("rules-mode-all"),
                        MatchMode::Any => tr!("rules-mode-any"),
                    },
                    picked == Some(Pick::Mode),
                    th,
                )
                .on_click(pick_at(Pick::Mode, cx)),
            )
            .child(tr!("rules-editor-of-these"));
        let conditions = e
            .conditions
            .iter()
            .enumerate()
            .map(|(ix, row)| {
                let field = select_box(
                    ("rule-field", ix),
                    field_label(row.field),
                    picked == Some(Pick::Field(ix)),
                    th,
                )
                .map(|d| if narrow { d.flex_1() } else { d.w(px(120.0)) })
                .on_click(pick_at(Pick::Field(ix), cx));
                let what: AnyElement = if row.field == Field::Tab {
                    let tab = select_box(
                        ("rule-tab", ix),
                        tab_label(row.tab),
                        picked == Some(Pick::Tab(ix)),
                        th,
                    )
                    .w(px(150.0))
                    .on_click(pick_at(Pick::Tab(ix), cx));
                    // Fills the row, so its remove button sits at the end.
                    div().flex_1().child(tab).into_any_element()
                } else if row.field.is_yes_no() {
                    let has = select_box(
                        ("rule-has", ix),
                        if row.has {
                            tr!("rules-has-yes")
                        } else {
                            tr!("rules-has-no")
                        },
                        picked == Some(Pick::Has(ix)),
                        th,
                    )
                    .w(px(150.0))
                    .on_click(pick_at(Pick::Has(ix), cx));
                    div().flex_1().child(has).into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .flex_1()
                        .min_w(px(if narrow { 200.0 } else { 0.0 }))
                        .gap(px(8.0))
                        .child(
                            select_box(
                                ("rule-comparator", ix),
                                comparator_label(row.comparator),
                                picked == Some(Pick::Comparator(ix)),
                                th,
                            )
                            // As wide as its label, so "matches the
                            // pattern" shows in full.
                            .min_w(px(192.0))
                            .on_click(pick_at(Pick::Comparator(ix), cx)),
                        )
                        .child(
                            text_area_field(("rule-value", ix), &row.value, th, cx)
                                .flex_1()
                                .min_w(px(160.0)),
                        )
                        .into_any_element()
                };
                part_row(
                    field,
                    what,
                    remove_button(("rule-condition-remove", ix), th).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.edit_rule(
                                |e| {
                                    if ix < e.conditions.len() {
                                        e.conditions.remove(ix);
                                    }
                                },
                                cx,
                            )
                        },
                    )),
                    narrow,
                )
            })
            .collect::<Vec<_>>();
        let actions = e
            .actions
            .iter()
            .enumerate()
            .map(|(ix, row)| {
                let kind = select_box(
                    ("rule-action", ix),
                    row.kind.label(),
                    picked == Some(Pick::Action(ix)),
                    th,
                )
                .map(|d| if narrow { d.flex_1() } else { d.w(px(270.0)) })
                .on_click(pick_at(Pick::Action(ix), cx));
                let what: AnyElement = match row.kind {
                    ActionKind::Move | ActionKind::AddLabel => {
                        // With several accounts, named with its account.
                        let chosen = row.folder.and_then(|f| {
                            self.rule_folders_of(e, row.kind, Some(f))
                                .into_iter()
                                .find(|(id, _)| *id == f)
                                .map(|(_, name)| name)
                                .or_else(|| self.rule_folder_name(f))
                        });
                        let empty = chosen.is_none();
                        select_box(
                            ("rule-folder", ix),
                            chosen.unwrap_or_else(|| {
                                if row.kind == ActionKind::AddLabel {
                                    tr!("rules-editor-choose-label")
                                } else {
                                    tr!("rules-editor-choose-folder")
                                }
                            }),
                            picked == Some(Pick::Folder(ix)),
                            th,
                        )
                        .flex_1()
                        // Keeps its width; a long name ends in …
                        .min_w_0()
                        .when(empty, |d| d.text_color(rgba(th.text_faint)))
                        .on_click(pick_at(Pick::Folder(ix), cx))
                        .into_any_element()
                    }
                    ActionKind::Forward => text_field(("rule-text", ix), &row.text, 36.0, th, cx)
                        .flex_1()
                        .into_any_element(),
                    ActionKind::MarkReadAfter => div()
                        .flex_1()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(text_field(("rule-text", ix), &row.text, 36.0, th, cx).w(px(80.0)))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("rules-editor-days")),
                        )
                        .into_any_element(),
                    _ => div().flex_1().into_any_element(),
                };
                part_row(
                    kind,
                    what,
                    remove_button(("rule-action-remove", ix), th).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.edit_rule(
                                |e| {
                                    if ix < e.actions.len() {
                                        e.actions.remove(ix);
                                    }
                                },
                                cx,
                            )
                        },
                    )),
                    narrow,
                )
            })
            .collect::<Vec<_>>();
        let stop = div()
            .id("rule-stop")
            .focus_ring(th)
            .mt(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(6.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| this.edit_rule(|e| e.stop = !e.stop, cx)))
            .child(checkbox("rule-stop-box", Check::from(e.stop), th))
            .child(tr!("rules-editor-stop"));
        let accounts = div()
            .mt(px(12.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .text_size(px(14.0))
            .child(tr!("rules-editor-accounts"))
            .child(
                select_box_with(
                    "rule-accounts",
                    self.rule_accounts_label(&e.accounts, th),
                    picked == Some(Pick::Accounts),
                    th,
                )
                .max_w_full()
                .on_click(pick_at(Pick::Accounts, cx)),
            );
        let panel = self.rule_preview_panel(e, th, cx);
        let error = e.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(8.0))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .text_color(rgba(th.error))
                .child(icon("warning", th.error, 18.0))
                .child(self.copyable(err, th).flex_1().min_w_0())
        });
        let buttons = self.rule_editor_buttons(e, th, cx);
        let scroll = div()
            .id("rule-editor-body")
            // Shrinks to the card's height so it scrolls instead of running
            // past the window.
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .child(
                // A scrolling column shrinks its children otherwise.
                div()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .px(px(if narrow { 16.0 } else { 26.0 }))
                    .pt(px(22.0))
                    .pb(px(4.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .line_height(px(30.0))
                            .mb(px(14.0))
                            .child(title),
                    )
                    .child(name)
                    .child(when)
                    .child(
                        div()
                            .mt(px(12.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .children(conditions),
                    )
                    .child(
                        add_link("rule-add-condition", tr!("rules-editor-add-condition"), th)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.add_rule_condition(window, cx)
                            })),
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(14.0))
                            .child(tr!("rules-editor-then")),
                    )
                    .child(
                        div()
                            .mt(px(10.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .children(actions),
                    )
                    .child(
                        add_link("rule-add-action", tr!("rules-editor-add-action"), th).on_click(
                            cx.listener(|this, _, window, cx| this.add_rule_action(window, cx)),
                        ),
                    )
                    .child(stop)
                    .child(accounts)
                    .child(panel),
            );
        // Save and Cancel stay below the scrolling part.
        div()
            .min_h_0()
            .flex()
            .flex_col()
            .child(scroll)
            .child(
                div()
                    .flex_none()
                    .px(px(if narrow { 16.0 } else { 26.0 }))
                    .pb(px(20.0))
                    .children(error)
                    .child(buttons),
            )
            .into_any_element()
    }

    /// The accounts chosen, with their colors: the address of one, or
    /// how many.
    fn rule_accounts_label(&self, accounts: &[i64], th: &Theme) -> AnyElement {
        let addresses: Vec<String> = accounts
            .iter()
            .filter_map(|a| self.account_address(AccountId(*a)))
            .collect();
        let dots = addresses.iter().map(|a| dot(self.account_color(a, th)));
        let text = match addresses.as_slice() {
            [] => tr!("rules-editor-accounts-none"),
            [one] => one.clone(),
            many => tr!("rules-editor-accounts-many", count = many.len()),
        };
        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .children(dots)
            .child(div().ml(px(4.0)).truncate().child(text))
            .into_any_element()
    }

    /// The light panel: how much recent mail the rule matches, "Also
    /// apply to these", and where it runs.
    fn rule_preview_panel(&self, e: &RuleEditor, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let rule = e.build(false, cx).ok();
        let query = rule.as_ref().and_then(|r| search_query(r, PREVIEW_DAYS));
        let count: AnyElement = match e.preview {
            Preview::Count(count) => {
                let mails = tr!("rules-editor-mails", count = count);
                let text = tr!(
                    "rules-editor-matches",
                    mails = mails.as_str(),
                    days = PREVIEW_DAYS
                );
                let bold: Vec<_> = text
                    .find(&mails)
                    .map(|at| {
                        (
                            at..at + mails.len(),
                            HighlightStyle {
                                font_weight: Some(FontWeight::BOLD),
                                ..Default::default()
                            },
                        )
                    })
                    .into_iter()
                    .collect();
                StyledText::new(SharedString::from(text))
                    .with_highlights(bold)
                    .into_any_element()
            }
            Preview::Counting => tr!("rules-editor-counting").into_any_element(),
            Preview::Nothing => tr!(
                "rules-editor-matches",
                mails = tr!("rules-editor-mails", count = 0),
                days = PREVIEW_DAYS
            )
            .into_any_element(),
        };
        let matched = match e.preview {
            Preview::Count(n) => n,
            _ => 0,
        };
        let show = query.filter(|_| matched > 0).map(|query| {
            div()
                .id("rule-show")
                .focus_ring(th)
                .rounded(px(4.0))
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .hover(|s| s.underline())
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.show_rule_mail(query.clone(), window, cx)
                }))
                .child(tr!("rules-editor-show"))
        });
        let apply = (matched > 0).then(|| {
            div()
                .id("rule-also-apply")
                .focus_ring(th)
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(e) = &mut this.rule_editor {
                        e.also_apply = !e.also_apply;
                    }
                    cx.notify();
                }))
                .child(checkbox(
                    "rule-also-apply-box",
                    Check::from(e.also_apply),
                    th,
                ))
                .child(tr!("rules-editor-also-apply", count = matched))
        });
        div()
            .mt(px(16.0))
            .px(px(14.0))
            .py(px(12.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .rounded(px(10.0))
            .bg(rgba(fade(th.accent, 0.06)))
            .text_size(px(14.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_x(px(8.0))
                    .child(icon("search", th.text_dim, 16.0))
                    .child(count)
                    .children(show),
            )
            .children(apply)
            .child(
                div()
                    .text_size(px(12.0))
                    .line_height(px(17.0))
                    .text_color(rgba(th.text_faint))
                    .child(runs_text(&e.rule, |id| self.rule_folder_name(id))),
            )
            .into_any_element()
    }

    /// Delete rule on the left (asking first), Cancel and Save on the
    /// right.
    fn rule_editor_buttons(
        &self,
        e: &RuleEditor,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row = div()
            .mt(px(20.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0));
        if e.asking_delete {
            return row
                .child(
                    div()
                        .flex_1()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(tr!("rules-editor-delete-ask")),
                )
                .child(
                    text_button("rule-delete-keep", tr!("rules-editor-delete-keep"), th)
                        .focus_ring(th)
                        .on_click(cx.listener(|this, _, _, cx| {
                            if let Some(e) = &mut this.rule_editor {
                                e.asking_delete = false;
                            }
                            cx.notify();
                        })),
                )
                .child(
                    filled_button(
                        "rule-delete-confirm",
                        tr!("rules-editor-delete-confirm"),
                        th,
                    )
                    .focus_ring_filled(th)
                    .bg(rgba(th.error))
                    .on_click(cx.listener(|this, _, _, cx| this.delete_rule_edit(cx))),
                )
                .into_any_element();
        }
        let busy = e.busy;
        row.when(e.rule.id != 0, |d| {
            d.child(
                text_button("rule-delete", tr!("rules-editor-delete"), th)
                    .focus_ring(th)
                    .text_color(rgba(th.error))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(e) = &mut this.rule_editor {
                            e.asking_delete = true;
                            e.pick = None;
                        }
                        cx.notify();
                    })),
            )
        })
        .child(div().flex_1())
        .child(
            text_button("rule-cancel", tr!("rules-editor-cancel"), th)
                .focus_ring(th)
                .on_click(cx.listener(|this, _, _, cx| this.close_rule_editor(cx))),
        )
        .child(
            filled_button(
                "rule-save",
                if busy {
                    tr!("rules-editor-saving")
                } else {
                    tr!("rules-editor-save")
                },
                th,
            )
            .focus_ring_filled(th)
            .when(busy, |d| d.opacity(0.6).cursor_default())
            .on_click(cx.listener(|this, _, _, cx| this.save_rule_edit(cx))),
        )
        .into_any_element()
    }

    /// The open menu of the editor, over everything, with a scrim that
    /// closes it.
    fn render_rule_pick(
        &self,
        e: &RuleEditor,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let (pick, at) = e.pick?;
        let item = |id: ElementId, label: String, on: bool| {
            crate::widgets::menu_item(id, &label, th)
                .gap(px(10.0))
                .when(on, |d| {
                    d.font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(th.accent))
                })
        };
        let items: Vec<AnyElement> = match pick {
            Pick::Mode => [MatchMode::All, MatchMode::Any]
                .into_iter()
                .enumerate()
                .map(|(n, mode)| {
                    item(
                        ("rule-pick-mode", n).into(),
                        match mode {
                            MatchMode::All => tr!("rules-mode-all"),
                            MatchMode::Any => tr!("rules-mode-any"),
                        },
                        e.mode == mode,
                    )
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.edit_rule(|e| e.mode = mode, cx)),
                    )
                    .into_any_element()
                })
                .collect(),
            Pick::Field(ix) => FIELDS
                .into_iter()
                .enumerate()
                .map(|(n, field)| {
                    let on = e.conditions.get(ix).is_some_and(|r| r.field == field);
                    item(("rule-pick-field", n).into(), field_label(field), on)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.edit_rule(
                                |e| {
                                    if let Some(row) = e.conditions.get_mut(ix) {
                                        row.field = field;
                                    }
                                },
                                cx,
                            )
                        }))
                        .into_any_element()
                })
                .collect(),
            Pick::Comparator(ix) => COMPARATORS
                .into_iter()
                .enumerate()
                .map(|(n, comparator)| {
                    let on = e
                        .conditions
                        .get(ix)
                        .is_some_and(|r| r.comparator == comparator);
                    item(
                        ("rule-pick-comparator", n).into(),
                        comparator_label(comparator),
                        on,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.edit_rule(
                            |e| {
                                if let Some(row) = e.conditions.get_mut(ix) {
                                    row.comparator = comparator;
                                }
                            },
                            cx,
                        )
                    }))
                    .into_any_element()
                })
                .collect(),
            Pick::Has(ix) => [true, false]
                .into_iter()
                .map(|has| {
                    let on = e.conditions.get(ix).is_some_and(|r| r.has == has);
                    item(
                        ("rule-pick-has", has as usize).into(),
                        if has {
                            tr!("rules-has-yes")
                        } else {
                            tr!("rules-has-no")
                        },
                        on,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.edit_rule(
                            |e| {
                                if let Some(row) = e.conditions.get_mut(ix) {
                                    row.has = has;
                                }
                            },
                            cx,
                        )
                    }))
                    .into_any_element()
                })
                .collect(),
            Pick::Tab(ix) => MailCategory::ALL
                .into_iter()
                .enumerate()
                .map(|(n, tab)| {
                    let on = e.conditions.get(ix).is_some_and(|r| r.tab == tab);
                    item(("rule-pick-tab", n).into(), tab_label(tab), on)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.edit_rule(
                                |e| {
                                    if let Some(row) = e.conditions.get_mut(ix) {
                                        row.tab = tab;
                                    }
                                },
                                cx,
                            )
                        }))
                        .into_any_element()
                })
                .collect(),
            Pick::Action(ix) => ActionKind::ALL
                .into_iter()
                .enumerate()
                // Add label only where there are labels: a Gmail account.
                .filter(|(_, kind)| {
                    *kind != ActionKind::AddLabel
                        || e.actions.get(ix).is_some_and(|r| r.kind == *kind)
                        || e.accounts.iter().any(|a| self.tree.is_gmail(AccountId(*a)))
                })
                .map(|(n, kind)| {
                    let on = e.actions.get(ix).is_some_and(|r| r.kind == kind);
                    item(("rule-pick-action", n).into(), kind.label(), on)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.set_rule_action(ix, kind, window, cx)
                        }))
                        .into_any_element()
                })
                .collect(),
            Pick::Folder(ix) => {
                let row = e.actions.get(ix)?;
                self.rule_folders_of(e, row.kind, row.folder)
                    .into_iter()
                    .enumerate()
                    .map(|(n, (folder, name))| {
                        item(
                            ("rule-pick-folder", n).into(),
                            name,
                            row.folder == Some(folder),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.edit_rule(
                                |e| {
                                    if let Some(row) = e.actions.get_mut(ix) {
                                        row.folder = Some(folder);
                                    }
                                },
                                cx,
                            )
                        }))
                        .into_any_element()
                    })
                    .collect()
            }
            Pick::Accounts => self
                .accounts
                .iter()
                .filter(|a| a.kind.is_mail())
                .enumerate()
                .map(|(n, account)| {
                    let id = account.id.0;
                    let on = e.accounts.contains(&id);
                    crate::widgets::menu_item(("rule-pick-account", n), "", th)
                        .gap(px(10.0))
                        .child(checkbox(("rule-pick-account-box", n), Check::from(on), th))
                        .child(dot(self.account_color(&account.address, th)))
                        .child(account.address.clone())
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(e) = &mut this.rule_editor {
                                if let Some(at) = e.accounts.iter().position(|a| *a == id) {
                                    e.accounts.remove(at);
                                } else {
                                    e.accounts.push(id);
                                }
                                // Folders of an account no longer chosen
                                // are chosen again.
                                let keep: Vec<i64> = e.accounts.clone();
                                let tree = &this.tree;
                                for row in &mut e.actions {
                                    if row.folder.is_some_and(|f| {
                                        tree.account_of(FolderId(f))
                                            .is_none_or(|a| !keep.contains(&a.0))
                                    }) {
                                        row.folder = None;
                                    }
                                }
                            }
                            // The menu stays open for more.
                            this.rule_changed(cx);
                        }))
                        .into_any_element()
                })
                .collect(),
        };
        let close = cx.listener(|this, _, _, cx| {
            cx.stop_propagation();
            if let Some(e) = &mut this.rule_editor {
                e.pick = None;
            }
            cx.notify();
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("rule-pick-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close),
                    )
                    .with_priority(5),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(
                                menu(th)
                                    .id("rule-pick")
                                    .occlude()
                                    .max_h(px(360.0))
                                    .overflow_y_scroll()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .children(items),
                            ),
                    )
                    .with_priority(6),
                )
                .into_any_element(),
        )
    }
}

/// The placeholder of an action row's field.
fn action_hint(kind: ActionKind) -> String {
    match kind {
        ActionKind::Forward => tr!("rules-editor-forward-hint"),
        _ => String::new(),
    }
}

fn action_of(kind: ActionKind, folder: Option<i64>, text: &str) -> Action {
    let folder = folder.unwrap_or(0);
    match kind {
        ActionKind::Move => Action::Move { folder },
        ActionKind::Archive => Action::Archive,
        ActionKind::Trash => Action::Trash,
        ActionKind::MarkRead => Action::MarkRead,
        ActionKind::Star => Action::Star,
        ActionKind::MarkImportant => Action::MarkImportant,
        ActionKind::AddLabel => Action::AddLabel { folder },
        ActionKind::Forward => Action::Forward {
            to: text.to_owned(),
        },
        ActionKind::DontNotify => Action::DontNotify,
        ActionKind::MarkReadAfter => Action::MarkReadAfter {
            days: text.parse().unwrap_or(1),
        },
    }
}

/// A click that opens menu `pick` where it happened.
fn pick_at(
    pick: Pick,
    cx: &mut Context<MailWindow>,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    cx.listener(move |this, event: &ClickEvent, _, cx| {
        this.open_rule_pick(pick, event.position(), cx)
    })
}

/// An account's color, as a dot.
pub(super) fn dot(color: u32) -> AnyElement {
    div()
        .flex_none()
        .size(px(8.0))
        .rounded_full()
        .bg(rgba(color))
        .into_any_element()
}

/// A box that opens a menu: what is chosen, and a down arrow.
fn select_box(
    id: impl Into<ElementId>,
    label: String,
    open: bool,
    th: &Theme,
) -> Stateful<gpui::Div> {
    select_box_with(
        id,
        div()
            .flex_1()
            .min_w_0()
            .truncate()
            .child(label)
            .into_any_element(),
        open,
        th,
    )
}

/// A [`select_box`] showing `content`.
fn select_box_with(
    id: impl Into<ElementId>,
    content: AnyElement,
    open: bool,
    th: &Theme,
) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .focus_ring(th)
        .flex_none()
        .h(px(36.0))
        .pl(px(12.0))
        .pr(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(rgba(if open { th.accent } else { th.outline }))
        .text_size(px(14.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(content)
        .child(icon("chevron-down", th.text_dim, 16.0))
}

/// A text field of the editor, edged in the accent while it has the
/// keys.
fn text_field(
    id: impl Into<ElementId>,
    input: &Entity<TextInput>,
    height: f32,
    th: &Theme,
    cx: &App,
) -> Stateful<gpui::Div> {
    crate::widgets::field(id, &input.focus_handle(cx), th)
        .h(px(height))
        .flex()
        .items_center()
        .child(div().flex_1().min_w_0().child(input.clone()))
}

/// A [`text_field`] for a [`TextArea`]: at least as tall as a field and
/// taller as its text wraps.
fn text_area_field(
    id: impl Into<ElementId>,
    area: &Entity<TextArea>,
    th: &Theme,
    cx: &App,
) -> Stateful<gpui::Div> {
    crate::widgets::field(id, &area.focus_handle(cx), th)
        .min_h(px(36.0))
        .py(px(7.0))
        .flex()
        .items_center()
        .line_height(px(20.0))
        .child(div().flex_1().min_w_0().child(area.clone()))
}

/// A condition or action row: its menu, what goes with it, and ✕. On a
/// narrow dialog the second part wraps under the first.
fn part_row(
    first: Stateful<gpui::Div>,
    rest: AnyElement,
    remove: Stateful<gpui::Div>,
    narrow: bool,
) -> AnyElement {
    if narrow {
        return div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(8.0))
                    .child(first)
                    .child(remove),
            )
            .child(rest)
            .into_any_element();
    }
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(8.0))
        .child(first)
        .child(rest)
        .child(remove)
        .into_any_element()
}

fn remove_button(id: impl Into<ElementId>, th: &Theme) -> Stateful<gpui::Div> {
    icon_button(id, "close", 18.0, th)
        .flex_none()
        .size(px(32.0))
        // Centred on the 36 px boxes of its row when rows align at the top.
        .mt(px(2.0))
        .focus_ring(th)
        .tooltip(tip(tr!("rules-editor-remove"), th))
}

/// "+ Add a condition" and "+ Add an action".
fn add_link(id: &'static str, label: String, th: &Theme) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .focus_ring(th)
        .mt(px(6.0))
        .h(px(32.0))
        .px(px(6.0))
        .self_start()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(4.0))
        .rounded(px(6.0))
        .text_size(px(14.0))
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
        .child(icon("add", th.accent, 16.0))
        .child(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(conditions: Vec<(Field, Comparator, &str)>, actions: Vec<Action>) -> Rule {
        Rule {
            name: "Test".into(),
            conditions: conditions
                .into_iter()
                .map(|(field, comparator, value)| Condition {
                    field,
                    comparator,
                    value: value.into(),
                })
                .collect(),
            actions,
            accounts: vec![1],
            ..Rule::default()
        }
    }

    #[test]
    fn summarizes_a_rule_in_one_line() {
        let r = rule(
            vec![
                (Field::From, Comparator::Contains, "substack.com"),
                (Field::Subject, Comparator::NotContains, "invoice"),
            ],
            vec![Action::Archive, Action::AddLabel { folder: 7 }],
        );
        let name = |id: i64| (id == 7).then(|| "Reading".to_owned());
        let line = summary(&r, name);
        assert!(line.contains("From contains substack.com"), "{line}");
        assert!(line.contains(" and "), "{line}");
        assert!(line.contains("Subject doesn't contain invoice"), "{line}");
        assert!(line.contains("→"), "{line}");
        assert!(line.contains("skip the inbox"), "{line}");
        assert!(line.contains("label Reading"), "{line}");

        let any = Rule {
            match_mode: MatchMode::Any,
            ..rule(
                vec![
                    (Field::HasAttachment, Comparator::Contains, "false"),
                    (Field::To, Comparator::Contains, "info@example.com"),
                ],
                vec![
                    Action::Move { folder: 9 },
                    Action::MarkReadAfter { days: 1 },
                ],
            )
        };
        let line = summary(&any, |_| None);
        assert!(line.contains("Has no attachment"), "{line}");
        assert!(line.contains(" or "), "{line}");
        assert!(line.contains("move to a folder that's gone"), "{line}");
        assert!(line.contains("mark read after 1 day"), "{line}");

        let grouped = Rule {
            match_mode: MatchMode::Any,
            ..rule(
                vec![
                    (Field::Subject, Comparator::Contains, "invoice"),
                    (Field::Subject, Comparator::Contains, "receipt"),
                    (Field::Subject, Comparator::Contains, "order"),
                    (Field::From, Comparator::Contains, "shop.com"),
                ],
                vec![Action::MarkRead, Action::Star, Action::DontNotify],
            )
        };
        assert_eq!(
            summary(&grouped, |_| None),
            "Subject contains invoice, receipt or order or From contains shop.com \
             → mark read, star, don't notify"
        );
    }

    #[test]
    fn summarizes_tabs_lists_plain_patterns_and_a_folder_per_account() {
        let r = rule(
            vec![
                (Field::From, Comparator::Matches, "irctc.co.in|railyatri.in"),
                (
                    Field::Subject,
                    Comparator::Matches,
                    "ticket|pnr|boarding pass",
                ),
                (Field::Tab, Comparator::NotContains, "promotions"),
                (Field::MailingList, Comparator::Equals, "no"),
            ],
            vec![
                Action::AddLabel { folder: 7 },
                Action::AddLabel { folder: 8 },
                Action::MarkImportant,
            ],
        );
        assert_eq!(
            summary(&r, |_| Some("Travel".to_owned())),
            "From contains irctc.co.in or railyatri.in, \
             Subject contains ticket, pnr or boarding pass, \
             Not in the Promotions tab and Not from a mailing list \
             → label Travel, mark important"
        );
        let long = rule(
            vec![(Field::Subject, Comparator::Matches, "a|b|c|d|e|f")],
            vec![Action::MarkRead],
        );
        assert_eq!(
            summary(&long, |_| None),
            "Subject contains a, b, c or 3 more → mark read"
        );
        let pattern = rule(
            vec![(Field::Subject, Comparator::Matches, "^re: .*")],
            vec![Action::MarkRead],
        );
        assert!(summary(&pattern, |_| None).contains("matches the pattern ^re: .*"));
    }

    #[test]
    fn known_errors_get_their_own_words() {
        assert!(error_text("folder 9999 no longer exists").contains("no longer exists"));
        assert!(error_text("the account cannot send mail").contains("can't send"));
        assert!(error_text("something odd").starts_with("Something odd"));
    }

    #[test]
    fn says_where_a_rule_runs_and_why() {
        let folder = |id: i64| (id == 7).then(|| "Receipts".to_owned());
        let mut rule = Rule {
            name: "R".into(),
            ..Rule::default()
        };
        assert_eq!(
            runs_text(&rule, folder),
            "Runs in Katna, while this computer is on."
        );
        rule.runs_on = RunsOn::Gmail;
        assert!(runs_text(&rule, folder).starts_with("Runs on Gmail, so it also works"));
        rule.runs_on = RunsOn::Sieve;
        assert!(runs_text(&rule, folder).starts_with("Runs on your mail server"));
        rule.runs_on = RunsOn::Katna;
        rule.runs_note = Some(RunsNote::Action {
            service: RunsOn::Gmail,
            action: Action::DontNotify,
        });
        assert_eq!(
            runs_text(&rule, folder),
            "Runs in Katna: Gmail filters can't do “don't notify”."
        );
        let note = |note| note_text(&note, &folder);
        assert_eq!(
            note(RunsNote::Action {
                service: RunsOn::Sieve,
                action: Action::AddLabel { folder: 7 },
            }),
            "Runs in Katna: your mail server's rules can't do “label Receipts”."
        );
        assert_eq!(
            note(RunsNote::Condition {
                service: RunsOn::Gmail,
                field: Field::From,
                comparator: Comparator::BeginsWith,
            }),
            "Runs in Katna: Gmail filters can't test “From begins with” as Katna does."
        );
        assert!(
            note(RunsNote::Failed {
                service: RunsOn::Sieve,
                error: "line 3: unknown command".into(),
            })
            .ends_with("didn't take it (Line 3: unknown command).")
        );
        for other in [
            RunsNote::Order {
                service: RunsOn::Sieve,
            },
            RunsNote::Stop {
                service: RunsOn::Gmail,
            },
            RunsNote::ForwardAddress { to: "a@b.c".into() },
            RunsNote::Folder {
                service: RunsOn::Gmail,
            },
            RunsNote::SignIn,
            RunsNote::OtherScript { name: "old".into() },
        ] {
            assert!(note(other).starts_with("Runs in Katna"));
        }
    }

    #[test]
    fn shows_them_with_a_search_when_one_can_say_it() {
        let r = rule(
            vec![
                (Field::From, Comparator::Contains, "substack.com"),
                (Field::Subject, Comparator::NotContains, "in\"voice"),
                (Field::HasAttachment, Comparator::Contains, "true"),
            ],
            vec![Action::Archive],
        );
        assert_eq!(
            search_query(&r, 30).as_deref(),
            Some(
                "from:\"substack.com\" -subject:\"invoice\" has:attachment in:inbox newer_than:30d"
            )
        );
        let any = Rule {
            match_mode: MatchMode::Any,
            ..rule(
                vec![
                    (Field::From, Comparator::Contains, "a.com"),
                    (Field::AnyRecipient, Comparator::Contains, "me@b.com"),
                ],
                vec![Action::Archive],
            )
        };
        assert_eq!(
            search_query(&any, 30).as_deref(),
            Some("(from:\"a.com\" OR to:\"me@b.com\") in:inbox newer_than:30d")
        );
        for (field, comparator) in [
            (Field::Body, Comparator::Contains),
            (Field::ReplyTo, Comparator::Contains),
            (Field::From, Comparator::BeginsWith),
            (Field::Subject, Comparator::Matches),
        ] {
            let r = rule(vec![(field, comparator, "x")], vec![Action::Archive]);
            assert_eq!(search_query(&r, 30), None, "{field:?} {comparator:?}");
        }
    }

    #[test]
    fn the_first_of_two_moves_gives_way() {
        let moves = [ActionKind::Move, ActionKind::Archive, ActionKind::Trash];
        for kind in ActionKind::ALL {
            assert_eq!(
                moves.contains(&kind),
                action_of(kind, Some(1), "1").moves(),
                "{kind:?}"
            );
        }
    }
}
