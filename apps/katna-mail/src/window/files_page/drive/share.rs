// SPDX-License-Identifier: GPL-3.0-or-later

//! The Share dialog of a drive item (`docs/ARCHITECTURE.md` §13.8): add
//! people with a role, see and change who has access, and open the link
//! to anyone. The drive's own sharing email is off unless ticked, as
//! Katna's big-file links are.

use gpui::{
    AnyElement, AppContext, ClipboardItem, Context, Entity, Focusable, FontWeight, MouseButton,
    MouseDownEvent, Pixels, Point, Subscription, Task, Window, div, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_dbus::{CloudAccess, CloudEntry};
use katna_i18n::tr;
use katna_search::contacts::Suggestion;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::super::super::MailWindow;
use super::super::super::compose::address_suggestions;
use crate::outgoing;
use crate::theme::{Theme, fade};
use crate::widgets::{
    Check, avatar, checkbox, elevation, filled_button, icon, menu, menu_item, outlined_button,
};

const WIDTH: f32 = 520.0;
/// Suggestions shown under the field.
const SUGGESTIONS: usize = 5;

/// Which of the dialog's dropdowns is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dropdown {
    /// The role the people being added get.
    NewRole,
    /// A grant's role, by its place in the list.
    Grant(usize),
    /// Who may open it with the link.
    Link,
}

/// The Share dialog, open on one drive item.
pub(in crate::window) struct Sharing {
    account: AccountId,
    entry: CloudEntry,
    onedrive: bool,
    input: Entity<TextInput>,
    _subscription: Subscription,
    /// People to add: name and address.
    people: Vec<(Option<String>, String)>,
    /// The role they get.
    role: &'static str,
    /// The drive emails them too.
    notify: bool,
    /// Who has access; `None` while it is read.
    access: Option<Result<Vec<CloudAccess>, String>>,
    suggestions: Vec<Suggestion>,
    dropdown: Option<(Dropdown, Point<Pixels>)>,
    busy: bool,
    error: Option<String>,
    shown: Spring,
    closing: bool,
    _load: Option<Task<()>>,
}

/// The roles a drive gives, most first.
fn roles(onedrive: bool) -> &'static [&'static str] {
    if onedrive {
        &["editor", "viewer"]
    } else {
        &["editor", "commenter", "viewer"]
    }
}

fn role_label(role: &str) -> String {
    match role {
        "owner" => tr!("files-share-role-owner"),
        "editor" => tr!("files-share-role-editor"),
        "commenter" => tr!("files-share-role-commenter"),
        _ => tr!("files-share-role-viewer"),
    }
}

impl MailWindow {
    /// Opens the Share dialog on drive item `entry`.
    pub(super) fn open_drive_share(
        &mut self,
        entry: &CloudEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.library.menu = None;
        if !self.drive_owned() {
            return;
        }
        let Some(account) = self.library.cloud.view.as_ref().map(|v| v.account) else {
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("files-share-add"), cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &input,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => {
                        let typed = this.share_typed(cx);
                        if !this.add_typed(cx) && typed.is_empty() {
                            this.share_now(cx);
                        }
                    }
                    InputEvent::Cancel => this.close_drive_share(cx),
                    InputEvent::Changed => this.share_typing(cx),
                },
            );
        window.focus(&input.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.library.cloud.sharing = Some(Sharing {
            account,
            entry: entry.clone(),
            onedrive: self.drive_is_onedrive(account),
            input,
            _subscription: subscription,
            people: Vec::new(),
            role: "viewer",
            notify: false,
            access: None,
            suggestions: Vec::new(),
            dropdown: None,
            busy: false,
            error: None,
            shown,
            closing: false,
            _load: None,
        });
        self.load_access(cx);
        cx.notify();
    }

    fn close_drive_share(&mut self, cx: &mut Context<Self>) {
        if let Some(sharing) = &mut self.library.cloud.sharing {
            if sharing.dropdown.take().is_some() {
                cx.notify();
                return;
            }
            sharing.closing = true;
            sharing.shown.set(0.0);
        }
        cx.notify();
    }

    /// Reads who has access again.
    fn load_access(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return;
        };
        let account = sharing.account.0;
        let id = sharing.entry.id.clone();
        sharing._load = Some(cx.spawn(async move |this, cx| {
            let got = crate::daemon::cloud_access(&connection, account, &id).await;
            this.update(cx, |this, cx| {
                if let Some(sharing) = &mut this.library.cloud.sharing
                    && sharing.entry.id == id
                {
                    sharing.access = Some(got);
                    sharing._load = None;
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn share_typed(&self, cx: &Context<Self>) -> String {
        self.library
            .cloud
            .sharing
            .as_ref()
            .map(|s| s.input.read(cx).text().trim().to_owned())
            .unwrap_or_default()
    }

    /// The field changed: a comma or space after an address makes it a
    /// chip, and the address book suggests.
    fn share_typing(&mut self, cx: &mut Context<Self>) {
        let typed = self.share_typed(cx);
        let ends = self.library.cloud.sharing.as_ref().is_some_and(|s| {
            let text = s.input.read(cx).text();
            text.ends_with([',', ';', ' '])
        });
        if ends && typed.trim_end_matches([',', ';']).contains('@') {
            self.add_typed(cx);
            return;
        }
        let Some(sharing) = &self.library.cloud.sharing else {
            return;
        };
        let skip: Vec<String> = sharing.people.iter().map(|(_, e)| e.clone()).collect();
        let account = sharing.account.0;
        let mut found = address_suggestions(&typed, Some(account), &skip, cx);
        found.truncate(SUGGESTIONS);
        if let Some(sharing) = &mut self.library.cloud.sharing {
            sharing.suggestions = found;
            sharing.error = None;
        }
        cx.notify();
    }

    /// Makes the typed addresses chips, or picks the first suggestion for
    /// a name; returns whether anything was added.
    fn add_typed(&mut self, cx: &mut Context<Self>) -> bool {
        let typed = self.share_typed(cx);
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return false;
        };
        let text = typed.trim_end_matches([',', ';']).trim();
        if text.is_empty() {
            return false;
        }
        let mut added: Vec<(Option<String>, String)> = match outgoing::parse_addresses(text) {
            Ok(list) => list
                .into_iter()
                .filter(|m| outgoing::valid_email(&m.email))
                .map(|m| (m.name, m.email))
                .collect(),
            Err(_) => Vec::new(),
        };
        if added.is_empty() {
            added.extend(
                text.split([',', ';', ' '])
                    .filter(|w| outgoing::valid_email(w))
                    .map(|w| (None, w.to_owned())),
            );
        }
        if added.is_empty()
            && let Some(first) = sharing.suggestions.first()
        {
            added.push((first.name.clone(), first.email.clone()));
        }
        if added.is_empty() {
            sharing.error = Some(tr!("files-share-not-address", text = text));
            cx.notify();
            return true;
        }
        for (name, email) in added {
            if !sharing
                .people
                .iter()
                .any(|(_, e)| e.eq_ignore_ascii_case(&email))
            {
                sharing.people.push((name, email));
            }
        }
        sharing.suggestions.clear();
        sharing.error = None;
        sharing.input.update(cx, |input, cx| input.set_text("", cx));
        cx.notify();
        true
    }

    fn pick_share_suggestion(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return;
        };
        let Some(found) = sharing.suggestions.get(ix).cloned() else {
            return;
        };
        if !sharing.people.iter().any(|(_, e)| *e == found.email) {
            sharing.people.push((found.name, found.email));
        }
        sharing.suggestions.clear();
        sharing.input.update(cx, |input, cx| input.set_text("", cx));
        window.focus(&sharing.input.focus_handle(cx), cx);
        cx.notify();
    }

    /// Shares with the people added, or closes when there are none.
    fn share_now(&mut self, cx: &mut Context<Self>) {
        self.add_typed(cx);
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return;
        };
        if sharing.busy {
            return;
        }
        if sharing.people.is_empty() {
            self.close_drive_share(cx);
            return;
        }
        sharing.busy = true;
        sharing.error = None;
        let account = sharing.account.0;
        let id = sharing.entry.id.clone();
        let addresses: Vec<String> = sharing.people.iter().map(|(_, e)| e.clone()).collect();
        let role = sharing.role;
        let notify = sharing.notify;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result =
                crate::daemon::cloud_grant(&connection, account, &id, &addresses, role, notify)
                    .await;
            this.update(cx, |this, cx| {
                let drive = this.drive_name(AccountId(account));
                let Some(sharing) = &mut this.library.cloud.sharing else {
                    return;
                };
                sharing.busy = false;
                match result {
                    Ok(refused) => {
                        sharing
                            .people
                            .retain(|(_, e)| refused.iter().any(|r| r == e));
                        let shared = addresses.len() - refused.len();
                        if refused.is_empty() {
                            this.close_drive_share(cx);
                            this.show_snackbar(tr!("files-share-shared", count = shared), None, cx);
                        } else {
                            sharing.error = Some(tr!(
                                "files-share-refused",
                                drive = drive.as_str(),
                                addresses = refused.join(", ")
                            ));
                            this.load_access(cx);
                        }
                    }
                    Err(err) => sharing.error = Some(tr!("files-share-failed", error = err)),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Changes grant `place` to `role`, or takes it away (empty), then
    /// reads the list again.
    fn set_grant(&mut self, place: usize, role: &'static str, cx: &mut Context<Self>) {
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return;
        };
        sharing.dropdown = None;
        let Some(Ok(all)) = &sharing.access else {
            return;
        };
        let Some(grant) = all.get(place) else {
            return;
        };
        let permission = grant.id.clone();
        self.change_sharing(
            move |connection, account, id| {
                Box::pin(async move {
                    crate::daemon::cloud_set_access(&connection, account, &id, &permission, role)
                        .await
                })
            },
            cx,
        );
    }

    /// Opens the item to anyone with the link as `role`, or closes it
    /// (empty).
    fn set_share_link(&mut self, role: &'static str, cx: &mut Context<Self>) {
        if let Some(sharing) = &mut self.library.cloud.sharing {
            sharing.dropdown = None;
        }
        self.change_sharing(
            move |connection, account, id| {
                Box::pin(async move {
                    crate::daemon::cloud_set_link(&connection, account, &id, role).await
                })
            },
            cx,
        );
    }

    /// Runs `change` against the drive, then reads who has access again.
    #[allow(clippy::type_complexity)]
    fn change_sharing(
        &mut self,
        change: impl FnOnce(
            zbus::Connection,
            i64,
            String,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>>>>
        + 'static,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(sharing) = &mut self.library.cloud.sharing else {
            return;
        };
        sharing.busy = true;
        sharing.error = None;
        let account = sharing.account.0;
        let id = sharing.entry.id.clone();
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = change(connection, account, id).await;
            this.update(cx, |this, cx| {
                if let Some(sharing) = &mut this.library.cloud.sharing {
                    sharing.busy = false;
                    if let Err(err) = result {
                        sharing.error = Some(tr!("files-share-failed", error = err));
                    }
                }
                this.load_access(cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Copies the item's link: the one for anyone when there is one.
    fn copy_share_link(&mut self, cx: &mut Context<Self>) {
        let Some(sharing) = &self.library.cloud.sharing else {
            return;
        };
        let anyone = match &sharing.access {
            Some(Ok(all)) => all
                .iter()
                .find(|a| a.who == "anyone" && !a.link.is_empty())
                .map(|a| a.link.clone()),
            _ => None,
        };
        let link = anyone.unwrap_or_else(|| sharing.entry.link.clone());
        if link.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(link));
        self.show_snackbar(tr!("files-drive-link-copied"), None, cx);
    }

    fn open_share_dropdown(&mut self, which: Dropdown, at: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(sharing) = &mut self.library.cloud.sharing {
            sharing.dropdown = if sharing.dropdown.is_some_and(|(d, _)| d == which) {
                None
            } else {
                Some((which, at))
            };
        }
        cx.notify();
    }

    // --- Drawing -------------------------------------------------------------

    /// The Share dialog, while it is open.
    pub(in crate::window) fn render_share_dialog(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let th = &th.lifted();
        let sharing = self.library.cloud.sharing.as_mut()?;
        let t = sharing.shown.tick(window, reduce);
        if sharing.closing && sharing.shown.settled() {
            self.library.cloud.sharing = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let sharing = self.library.cloud.sharing.as_ref()?;
        let drive = self.drive_name(sharing.account);
        let me = self
            .accounts
            .iter()
            .find(|a| a.id == sharing.account)
            .map(|a| a.address.to_lowercase())
            .unwrap_or_default();
        let focused = sharing.input.focus_handle(cx).is_focused(window);
        let focus = sharing.input.focus_handle(cx);

        // The people being added, then the box to type in, then their role.
        let chips = sharing
            .people
            .iter()
            .enumerate()
            .map(|(ix, (name, email))| {
                let label = name.clone().unwrap_or_else(|| email.clone());
                div()
                    .id(("files-share-chip", ix))
                    .h(px(28.0))
                    .pl(px(2.0))
                    .pr(px(4.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .rounded_full()
                    .border_1()
                    .border_color(rgba(th.divider))
                    .text_size(px(13.0))
                    .child(avatar(name.as_deref().unwrap_or(email), email, 24.0))
                    .child(div().max_w(px(220.0)).truncate().child(label))
                    .child(
                        div()
                            .id(("files-share-chip-x", ix))
                            .size(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(sharing) = &mut this.library.cloud.sharing
                                    && ix < sharing.people.len()
                                {
                                    sharing.people.remove(ix);
                                }
                                cx.notify();
                            }))
                            .child(icon("close", th.text_dim, 14.0)),
                    )
            });
        let role_button = div()
            .id("files-share-new-role")
            .flex_none()
            .h(px(32.0))
            .pl(px(10.0))
            .pr(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .rounded(px(6.0))
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    this.open_share_dropdown(Dropdown::NewRole, e.position, cx);
                }),
            )
            .child(role_label(sharing.role))
            .child(icon("drop-down", th.text_dim, 20.0));
        let field = div()
            .id("files-share-field")
            .mt(px(16.0))
            .min_h(px(48.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .rounded(px(8.0))
            // The focus ring is a pixel thicker, taken from the padding so
            // nothing inside moves.
            .map(|d| {
                if focused {
                    d.border_2().px(px(7.0)).py(px(5.0))
                } else {
                    d.border_1().px(px(8.0)).py(px(6.0))
                }
            })
            .border_color(rgba(if focused {
                th.accent
            } else {
                fade(th.text_faint, 0.8)
            }))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .children(chips)
            .child(
                div()
                    .flex_1()
                    .min_w(px(120.0))
                    .text_size(px(15.0))
                    .child(sharing.input.clone()),
            )
            .child(role_button);
        let suggestions = (!sharing.suggestions.is_empty()).then(|| {
            div()
                .mt(px(4.0))
                .py(px(4.0))
                .flex()
                .flex_col()
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .children(sharing.suggestions.iter().enumerate().map(|(ix, s)| {
                    let name = s.name.clone().unwrap_or_default();
                    div()
                        .id(("files-share-suggestion", ix))
                        .h(px(48.0))
                        .px(px(12.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .cursor_pointer()
                        .hover(|st| st.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.pick_share_suggestion(ix, window, cx);
                        }))
                        .child(avatar(&name, &s.email, 32.0))
                        .child(
                            div()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .when(!name.is_empty(), |d| {
                                    d.child(
                                        div().truncate().text_size(px(14.0)).child(name.clone()),
                                    )
                                })
                                .child(
                                    div()
                                        .truncate()
                                        .text_size(px(12.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(s.email.clone()),
                                ),
                        )
                }))
        });
        let adding = !sharing.people.is_empty();
        let notify = adding.then(|| {
            div()
                .id("files-share-notify")
                .mt(px(12.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .cursor_pointer()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(sharing) = &mut this.library.cloud.sharing {
                        sharing.notify = !sharing.notify;
                    }
                    cx.notify();
                }))
                .child(checkbox(
                    "files-share-notify-box",
                    Check::from(sharing.notify),
                    th,
                ))
                .child(tr!("files-share-notify", drive = drive.as_str()))
        });

        // Who has access, and the link.
        let heading = |text: String| {
            div()
                .mt(px(20.0))
                .mb(px(6.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text)
        };
        let (people, link_role): (Vec<AnyElement>, Option<String>) = match &sharing.access {
            None => (
                vec![
                    div()
                        .h(px(48.0))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(crate::widgets::spinner(
                            "files-share-loading",
                            th.text_dim,
                            18.0,
                        ))
                        .child(tr!("files-share-loading"))
                        .into_any_element(),
                ],
                None,
            ),
            Some(Err(err)) => (
                vec![
                    div()
                        .py(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.error))
                        .child(tr!("files-share-failed", error = err.clone()))
                        .into_any_element(),
                ],
                None,
            ),
            Some(Ok(all)) => {
                let link = all
                    .iter()
                    .find(|a| a.who == "anyone")
                    .map(|a| a.role.clone());
                let rows = all
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| a.who != "anyone")
                    .map(|(place, a)| self.render_grant(place, a, &me, th, cx))
                    .collect();
                (rows, link)
            }
        };
        let open = link_role.is_some();
        let link_text = match &link_role {
            Some(role) => tr!("files-share-anyone-about", role = role.as_str()),
            None => tr!("files-share-restricted-about"),
        };
        let general = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .size(px(36.0))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(if open {
                        fade(th.accent, 0.16)
                    } else {
                        th.hover
                    }))
                    .child(icon(
                        if open { "link" } else { "lock" },
                        if open { th.accent } else { th.text_dim },
                        18.0,
                    )),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id("files-share-link")
                            .flex()
                            .flex_row()
                            .items_center()
                            .self_start()
                            .pl(px(6.0))
                            .pr(px(2.0))
                            .ml(px(-6.0))
                            .rounded(px(6.0))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, e: &MouseDownEvent, _, cx| {
                                    cx.stop_propagation();
                                    this.open_share_dropdown(Dropdown::Link, e.position, cx);
                                }),
                            )
                            .child(if open {
                                tr!("files-share-anyone")
                            } else {
                                tr!("files-share-restricted")
                            })
                            .child(icon("drop-down", th.text_dim, 20.0)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(link_text),
                    ),
            );
        let error = sharing.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(err)
        });
        let busy = sharing.busy;
        let foot = div()
            .mt(px(24.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                outlined_button("files-share-copy", tr!("files-share-copy-link"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.copy_share_link(cx))),
            )
            .child(div().flex_1())
            .child(
                filled_button(
                    "files-share-go",
                    if busy {
                        tr!("files-share-sharing")
                    } else if adding {
                        tr!("files-share-share")
                    } else {
                        tr!("files-share-done")
                    },
                    th,
                )
                .when(busy, |d| d.opacity(0.6).cursor_default())
                .on_click(cx.listener(|this, _, _, cx| this.share_now(cx))),
            );
        let body = div()
            .id("files-share-body")
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .px(px(24.0))
            .pt(px(22.0))
            .pb(px(18.0))
            .child(
                div()
                    .text_size(px(22.0))
                    .line_height(px(30.0))
                    .truncate()
                    .child(tr!("files-share-title", name = sharing.entry.name.as_str())),
            )
            .child(field)
            .children(suggestions)
            .children(notify)
            .child(heading(tr!("files-share-people")))
            .children(people)
            .child(heading(tr!("files-share-general")))
            .child(general)
            .children(error)
            .child(foot);
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let card = div()
            .id("files-share")
            .track_focus(&self.dialog_focus)
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(240.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(super::super::super::PANEL_RADIUS))
            .map(|d| crate::widgets::frosted(d, th, th.surface, super::super::super::PANEL_RADIUS))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    if let Some(sharing) = &mut this.library.cloud.sharing
                        && sharing.dropdown.take().is_some()
                    {
                        cx.notify();
                    }
                }),
            )
            .child(body);
        let dropdown = sharing
            .dropdown
            .map(|(which, at)| self.render_share_dropdown(which, at, (vw, vh), th, cx));
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_start()
                .justify_center()
                // A fixed top, so the card grows down as the list arrives
                // instead of jumping.
                .pt(px(((vh - 640.0) / 2.0).max(24.0)))
                .child(
                    div()
                        .id("files-share-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_drive_share(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .children(dropdown)
                .into_any_element(),
        )
    }

    /// One line of who has access: their picture, name and address, and
    /// what they may do.
    fn render_grant(
        &self,
        place: usize,
        grant: &CloudAccess,
        me: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let you = !me.is_empty() && grant.address.to_lowercase() == me;
        let title = match grant.who.as_str() {
            "domain" => tr!("files-share-domain", domain = grant.address.as_str()),
            _ if grant.name.is_empty() => grant.address.clone(),
            _ if you => tr!("files-share-you", name = grant.name.as_str()),
            _ => grant.name.clone(),
        };
        let under = if grant.inherited {
            tr!("files-share-inherited")
        } else if grant.name.is_empty() || grant.who == "domain" {
            String::new()
        } else {
            grant.address.clone()
        };
        let fixed = grant.role == "owner" || grant.inherited;
        let role = div()
            .id(("files-share-grant-role", place))
            .flex_none()
            .h(px(32.0))
            .pl(px(10.0))
            // A fixed role lines up with the others' words, not their arrows.
            .pr(px(if fixed { 24.0 } else { 4.0 }))
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(6.0))
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim))
            .child(role_label(&grant.role))
            .when(!fixed, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, e: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                            this.open_share_dropdown(Dropdown::Grant(place), e.position, cx);
                        }),
                    )
                    .child(icon("drop-down", th.text_dim, 20.0))
            });
        div()
            .h(px(52.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(avatar(&grant.name, &grant.address, 36.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(div().truncate().text_size(px(14.0)).child(title))
                    .when(!under.is_empty(), |d| {
                        d.child(
                            div()
                                .truncate()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(under),
                        )
                    }),
            )
            .child(role)
            .into_any_element()
    }

    fn render_share_dropdown(
        &self,
        which: Dropdown,
        at: Point<Pixels>,
        (vw, vh): (f32, f32),
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let sharing = self.library.cloud.sharing.as_ref();
        let onedrive = sharing.is_some_and(|s| s.onedrive);
        let tick = |on: bool| {
            div()
                .w(px(20.0))
                .flex_none()
                .children(on.then(|| icon("check", th.accent, 18.0)))
        };
        let mut items: Vec<AnyElement> = Vec::new();
        match which {
            Dropdown::NewRole => {
                let now = sharing.map_or("viewer", |s| s.role);
                for (ix, &role) in roles(onedrive).iter().enumerate() {
                    items.push(
                        menu_item(("files-share-pick-role", ix), &role_label(role), th)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(sharing) = &mut this.library.cloud.sharing {
                                    sharing.role = role;
                                    sharing.dropdown = None;
                                }
                                cx.notify();
                            }))
                            .justify_between()
                            .children([tick(now == role).into_any_element()])
                            .into_any_element(),
                    );
                }
            }
            Dropdown::Grant(place) => {
                let now = sharing
                    .and_then(|s| match &s.access {
                        Some(Ok(all)) => all.get(place).map(|a| a.role.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                for (ix, &role) in roles(onedrive).iter().enumerate() {
                    items.push(
                        menu_item(("files-share-grant-pick", ix), &role_label(role), th)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.set_grant(place, role, cx);
                            }))
                            .justify_between()
                            .children([tick(now == role).into_any_element()])
                            .into_any_element(),
                    );
                }
                items.push(
                    div()
                        .my(px(6.0))
                        .h(px(1.0))
                        .bg(rgba(th.divider))
                        .into_any_element(),
                );
                items.push(
                    menu_item("files-share-remove", &tr!("files-share-remove"), th)
                        .on_click(cx.listener(move |this, _, _, cx| this.set_grant(place, "", cx)))
                        .into_any_element(),
                );
            }
            Dropdown::Link => {
                let now = sharing
                    .and_then(|s| match &s.access {
                        Some(Ok(all)) => all
                            .iter()
                            .find(|a| a.who == "anyone")
                            .map(|a| a.role.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                items.push(
                    menu_item("files-share-link-off", &tr!("files-share-restricted"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.set_share_link("", cx)))
                        .justify_between()
                        .children([tick(now.is_empty()).into_any_element()])
                        .into_any_element(),
                );
                for (ix, &role) in roles(onedrive).iter().rev().enumerate() {
                    items.push(
                        menu_item(
                            ("files-share-link-role", ix),
                            &tr!("files-share-anyone-can", role = role),
                            th,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_share_link(role, cx);
                        }))
                        .justify_between()
                        .children([tick(now == role).into_any_element()])
                        .into_any_element(),
                    );
                }
            }
        }
        let count = items.len() as f32;
        let wide = matches!(which, Dropdown::Link);
        let (w, h) = (if wide { 280.0 } else { 200.0 }, 24.0 + 32.0 * count);
        let x = unpx(at.x).min(vw - w - 8.0).max(8.0);
        // Below the click when it fits, else above it.
        let below = unpx(at.y) + 8.0;
        let y = if below + h <= vh - 8.0 {
            below
        } else {
            (unpx(at.y) - h - 8.0).max(8.0)
        };
        menu(th)
            .id("files-share-dropdown")
            .occlude()
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(w))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .children(items)
            .into_any_element()
    }
}
