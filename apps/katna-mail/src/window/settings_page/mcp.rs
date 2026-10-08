// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > MCP server (`[mcp]`): whether AI assistants on this
//! computer may use the mail through `katnactl mcp`, whether they may
//! save drafts, which accounts they see, how to connect each assistant,
//! and what they did lately (`katna_core::mcp_activity`). Off until
//! turned on (MCP settings study, 2026-10-07).

use gpui::{AnyElement, ClipboardItem, Context, Div, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_core::mcp_activity::{self, Activity, ActivityKind};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{radius, space, state, text};

use super::{MailWindow, chip};
use crate::theme::Theme;
use crate::widgets::{ButtonStyle, button, icon};
use crate::window::settings::Change;

/// The lines of Recently shown at most.
const RECENT_SHOWN: usize = 8;

/// The assistants Connect an assistant has words for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum Client {
    #[default]
    ClaudeDesktop,
    ClaudeCode,
    LmStudio,
    Other,
}

impl Client {
    const ALL: [Self; 4] = [
        Self::ClaudeDesktop,
        Self::ClaudeCode,
        Self::LmStudio,
        Self::Other,
    ];

    fn label(self) -> String {
        match self {
            // Product names, the same in every language.
            Self::ClaudeDesktop => "Claude Desktop".to_owned(),
            Self::ClaudeCode => "Claude Code".to_owned(),
            Self::LmStudio => "LM Studio".to_owned(),
            Self::Other => tr!("mcp-client-other"),
        }
    }

    fn how(self) -> String {
        match self {
            Self::ClaudeDesktop => tr!("mcp-connect-claude-desktop"),
            Self::ClaudeCode => tr!("mcp-connect-claude-code"),
            Self::LmStudio => tr!("mcp-connect-lm-studio"),
            Self::Other => tr!("mcp-connect-other"),
        }
    }

    /// What to paste, for `command` (katnactl's path).
    fn text(self, command: &str) -> String {
        match self {
            Self::ClaudeDesktop | Self::LmStudio => config_json(command),
            Self::ClaudeCode => format!("claude mcp add katna -- {} mcp", shell_word(command)),
            Self::Other => format!("{} mcp", shell_word(command)),
        }
    }
}

/// The `mcpServers` entry Claude Desktop and LM Studio read.
fn config_json(command: &str) -> String {
    let command = serde_json::to_string(command).unwrap_or_default();
    format!(
        "{{\n  \"mcpServers\": {{\n    \"katna\": {{\n      \"command\": {command},\n      \"args\": [\"mcp\"]\n    }}\n  }}\n}}"
    )
}

/// `word` for a shell: quoted when it has spaces or quotes.
fn shell_word(word: &str) -> String {
    if word
        .chars()
        .any(|c| c.is_whitespace() || "'\"\\$`".contains(c))
    {
        format!("\"{}\"", word.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        word.to_owned()
    }
}

/// `katnactl` beside this program, as installed; its bare name when it
/// is not there (a build run from the source tree).
fn katnactl() -> String {
    let name = if cfg!(windows) {
        "katnactl.exe"
    } else {
        "katnactl"
    };
    std::env::current_exe()
        .ok()
        .map(|exe| exe.with_file_name(name))
        .filter(|path| path.is_file())
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| "katnactl".to_owned())
}

/// What the page keeps while open.
#[derive(Default)]
pub(super) struct McpPage {
    client: Client,
    /// What assistants did, newest first, read when the page opened.
    activity: Vec<Activity>,
    /// The text was just copied.
    copied: bool,
}

impl MailWindow {
    /// Reads Recently again.
    pub(super) fn load_mcp_activity(&mut self) {
        let activity = mcp_activity::read(&self.paths);
        if let Some(page) = &mut self.settings_page {
            page.mcp.activity = activity;
        }
    }

    /// A change of [`Change::McpOn`] and the like.
    pub(in crate::window) fn apply_mcp(&mut self, change: Change, cx: &mut Context<Self>) {
        let mcp = &mut self.config.mcp;
        match change {
            Change::McpOn(on) => mcp.enabled = on,
            Change::McpDrafts(on) => mcp.drafts = on,
            Change::McpAccount(id, shown) => {
                let Some(account) = self.accounts.iter().find(|a| a.id == id) else {
                    return;
                };
                mcp.set_shown(&account.address, shown);
            }
            _ => return,
        }
        self.save_config();
        cx.notify();
    }

    fn pick_mcp_client(&mut self, client: Client, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.mcp.client = client;
            page.mcp.copied = false;
        }
        cx.notify();
    }

    fn copy_mcp_text(&mut self, text: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        if let Some(page) = &mut self.settings_page {
            page.mcp.copied = true;
        }
        cx.notify();
    }

    fn clear_mcp_activity(&mut self, cx: &mut Context<Self>) {
        if let Err(err) = mcp_activity::clear(&self.paths) {
            self.show_snackbar(err.to_string(), None, cx);
        }
        self.load_mcp_activity();
        cx.notify();
    }

    /// The page.
    pub(super) fn mcp_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let mcp = &self.config.mcp;
        let on = mcp.enabled;
        let assistants = div()
            .flex()
            .flex_col()
            .child(self.switch_row(
                "page-mcp-on",
                tr!("mcp-assistants-switch"),
                tr!("mcp-assistants-switch-detail"),
                on,
                Change::McpOn(!on),
                th,
                cx,
            ))
            .when(!on, |d| {
                d.child(
                    div()
                        .px(px(space::S3))
                        .pt(px(space::S2))
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("mcp-assistants-off")),
                )
            });
        // The rest waits, dimmed, until assistants are let in; it can
        // still be set up first.
        let rest = div()
            .flex()
            .flex_col()
            .when(!on, |d| d.opacity(state::DISABLED))
            .child(self.row(
                tr!("mcp-drafts"),
                None,
                self.switch_row(
                    "page-mcp-drafts",
                    tr!("mcp-drafts-switch"),
                    tr!("mcp-drafts-switch-detail"),
                    mcp.drafts,
                    Change::McpDrafts(!mcp.drafts),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                tr!("mcp-accounts"),
                Some(&tr!("mcp-accounts-detail")),
                self.mcp_accounts(th, cx),
                th,
            ))
            .child(self.row(tr!("mcp-connect"), None, self.mcp_connect(th, cx), th))
            .child(self.row(
                tr!("mcp-recently"),
                Some(&tr!("mcp-recently-detail")),
                self.mcp_recently(th, cx),
                th,
            ));
        div()
            .flex()
            .flex_col()
            .child(self.row(tr!("mcp-assistants"), None, assistants, th))
            .child(rest)
            .into_any_element()
    }

    /// A switch for each mail account.
    fn mcp_accounts(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let mcp = &self.config.mcp;
        div()
            .flex()
            .flex_col()
            .children(
                self.accounts
                    .iter()
                    .filter(|a| a.kind.is_mail())
                    .map(|account| {
                        let shown = mcp.shows(&account.address);
                        let name = if account.display_name.trim().is_empty() {
                            account.address.clone()
                        } else {
                            account.display_name.clone()
                        };
                        let detail = if name == account.address {
                            String::new()
                        } else {
                            account.address.clone()
                        };
                        let id: AccountId = account.id;
                        self.switch_row(
                            ("page-mcp-account", id.0 as usize),
                            name,
                            detail,
                            shown,
                            Change::McpAccount(id, !shown),
                            th,
                            cx,
                        )
                    }),
            )
    }

    /// The assistants as chips, then what to do in the one picked and
    /// the text to paste, with Copy.
    fn mcp_connect(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let (picked, copied) = self
            .settings_page
            .as_ref()
            .map(|p| (p.mcp.client, p.mcp.copied))
            .unwrap_or_default();
        let chips = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(space::S2))
            .children(Client::ALL.into_iter().enumerate().map(|(n, client)| {
                chip(("page-mcp-client", n), client.label(), client == picked, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_mcp_client(client, cx)))
            }));
        let text = picked.text(&katnactl());
        let shown = text.clone();
        let mut pieces = self.ui_pieces(th);
        let code = self.ui_selectable(pieces.words(shown), &pieces);
        let copy = button("page-mcp-copy", ButtonStyle::Outlined, th)
            .map(|d| self.page_control(d, th, cx))
            .gap(px(space::S3))
            .child(icon(if copied { "check" } else { "copy" }, th.accent, 16.0))
            .child(if copied {
                tr!("mcp-copied")
            } else {
                tr!("mcp-copy")
            })
            .on_click(cx.listener(move |this, _, _, cx| this.copy_mcp_text(text.clone(), cx)));
        // Lined up with the text of the switch rows above.
        div()
            .px(px(space::S3))
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(chips)
            .child(
                div()
                    .text_size(px(text::SMALL))
                    .child(self.copyable(picked.how(), th)),
            )
            .child(
                div()
                    .px(px(space::S4))
                    .py(px(space::S3))
                    .rounded(px(radius::SM))
                    .border_1()
                    .border_color(rgba(th.outline))
                    .font_family("monospace")
                    .text_size(px(text::CAPTION))
                    .overflow_x_hidden()
                    .child(code),
            )
            .child(div().flex().flex_row().child(copy))
    }

    /// What assistants did lately, newest first, with Clear.
    fn mcp_recently(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let activity = self
            .settings_page
            .as_ref()
            .map(|p| p.mcp.activity.as_slice())
            .unwrap_or_default();
        if activity.is_empty() {
            return div().child(self.quiet_note(tr!("mcp-recently-none"), th));
        }
        let now = jiff::Timestamp::now().as_second();
        let lines = activity.iter().take(RECENT_SHOWN).map(|a| {
            let (name, said) = describe(a);
            let when = crate::format::ago(a.time, now).unwrap_or_else(|| {
                crate::format::local(a.time, &self.tz)
                    .map(crate::format::long_date)
                    .unwrap_or_default()
            });
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S4))
                .px(px(space::S3))
                .py(px(space::S2))
                .child(div().pt(px(space::S1)).child(icon(name, th.text_dim, 18.0)))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .text_size(px(text::BODY))
                        .child(self.copyable(said, th))
                        .when(!a.subject.is_empty(), |d| {
                            d.child(
                                div()
                                    .text_size(px(text::CAPTION))
                                    .text_color(rgba(th.text_faint))
                                    .child(self.copyable(a.subject.clone(), th)),
                            )
                        }),
                )
                .child(
                    div()
                        .flex_none()
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(th.text_faint))
                        .child(when),
                )
        });
        let clear = button("page-mcp-clear", ButtonStyle::Text, th)
            .map(|d| self.page_control(d, th, cx))
            .child(tr!("mcp-recently-clear"))
            .on_click(cx.listener(|this, _, _, cx| this.clear_mcp_activity(cx)));
        div()
            .flex()
            .flex_col()
            .children(lines)
            .child(div().flex().flex_row().pt(px(space::S2)).child(clear))
    }
}

/// The icon and the line of one thing an assistant did.
fn describe(a: &Activity) -> (&'static str, String) {
    let client = if a.client.trim().is_empty() {
        tr!("mcp-someone")
    } else {
        a.client.clone()
    };
    match a.kind {
        ActivityKind::Search if a.detail.trim().is_empty() => {
            ("search", tr!("mcp-did-search-all", client = client))
        }
        ActivityKind::Search => (
            "search",
            tr!("mcp-did-search", client = client, query = a.detail.clone()),
        ),
        ActivityKind::Read => (
            "mail",
            tr!("mcp-did-read", client = client, subject = a.detail.clone()),
        ),
        ActivityKind::Draft if a.detail.trim().is_empty() => {
            ("drafts", tr!("mcp-did-draft-nobody", client = client))
        }
        ActivityKind::Draft => (
            "drafts",
            tr!("mcp-did-draft", client = client, to = a.detail.clone()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_to_paste() {
        let json = Client::ClaudeDesktop.text("/usr/bin/katnactl");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed["mcpServers"]["katna"]["command"],
            "/usr/bin/katnactl"
        );
        assert_eq!(parsed["mcpServers"]["katna"]["args"][0], "mcp");
        let windows = Client::LmStudio.text(r"C:\Program Files\Katna\katnactl.exe");
        let parsed: serde_json::Value = serde_json::from_str(&windows).unwrap();
        assert_eq!(
            parsed["mcpServers"]["katna"]["command"],
            r"C:\Program Files\Katna\katnactl.exe"
        );
        assert_eq!(
            Client::ClaudeCode.text("/usr/bin/katnactl"),
            "claude mcp add katna -- /usr/bin/katnactl mcp"
        );
        assert_eq!(
            Client::Other.text(r"C:\Program Files\Katna\katnactl.exe"),
            r#""C:\\Program Files\\Katna\\katnactl.exe" mcp"#
        );
    }
}
