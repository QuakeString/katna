// SPDX-License-Identifier: GPL-3.0-or-later

//! The New label dialog, as in Gmail: a name, and optionally a folder to
//! nest it under. Opened from the "+" beside an account's name in the
//! navigation. The daemon creates the folder on the server (a label, on
//! Gmail); the navigation shows it once the store has it.
//!
//! The same dialog renames a folder or label the user made, from the
//! folder pane's right-click menu: its name filled in and selected.

use gpui::{
    AnyElement, Context, Entity, Focusable, FontWeight, Subscription, Window, div, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::FolderId;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;
use katna_ui::{InputEvent, TextInput};

use super::MailWindow;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, ScaledEdge, filled_button, radio};
use crate::{daemon, format};

const WIDTH: f32 = 420.0;

pub(super) struct NewLabel {
    account: AccountId,
    /// Gmail calls folders labels.
    gmail: bool,
    name: Entity<TextInput>,
    _subscription: Subscription,
    nest: bool,
    parent: Option<FolderId>,
    /// Folders it may go inside, with their paths.
    parents: Vec<(FolderId, String)>,
    /// Renaming this folder instead, with its name as it was.
    rename: Option<(FolderId, String)>,
    busy: bool,
    error: Option<String>,
    closing: bool,
    shown: Spring,
}

impl MailWindow {
    pub(super) fn open_new_label(
        &mut self,
        account: AccountId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let gmail = self.tree.is_gmail(account);
        let accent = rgba(self.theme(window).accent).into();
        let name = cx.new(|cx| {
            let mut input = TextInput::new(
                if gmail {
                    tr!("label-name-hint")
                } else {
                    tr!("label-folder-name-hint")
                },
                cx,
            );
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &name,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.create_label(cx),
                    InputEvent::Cancel => this.close_new_label(cx),
                    _ => {
                        if let Some(dialog) = &mut this.new_label {
                            dialog.error = None;
                        }
                        cx.notify();
                    }
                },
            );
        window.focus(&name.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.new_label = Some(NewLabel {
            account,
            gmail,
            name,
            _subscription: subscription,
            nest: false,
            parent: None,
            parents: self.tree.nest_targets(account),
            rename: None,
            busy: false,
            error: None,
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// Opens the dialog to rename `folder`, its name filled in and
    /// selected.
    pub(super) fn open_rename_label(
        &mut self,
        folder: FolderId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (Some(account), Some(node)) = (self.tree.account_of(folder), self.tree.node(folder))
        else {
            return;
        };
        let old = node.name.clone();
        self.open_new_label(account, window, cx);
        let Some(dialog) = &mut self.new_label else {
            return;
        };
        dialog.parents.clear();
        dialog.rename = Some((folder, old.clone()));
        dialog.name.update(cx, |input, cx| {
            input.set_text(old, cx);
            input.select_all_text(cx);
        });
        // Filling the name in is no change to clear an error for.
        dialog.error = None;
    }

    /// Makes the open dialog put the new folder or label inside `parent`.
    pub(super) fn nest_new_label(&mut self, parent: FolderId) {
        if let Some(dialog) = &mut self.new_label
            && dialog.parents.iter().any(|(id, _)| *id == parent)
        {
            dialog.nest = true;
            dialog.parent = Some(parent);
        }
    }

    fn close_new_label(&mut self, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.new_label
            && !dialog.busy
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
        }
        cx.notify();
    }

    fn new_label_ready(&self, cx: &Context<Self>) -> bool {
        self.new_label.as_ref().is_some_and(|dialog| {
            let name = dialog.name.read(cx).text().trim();
            !dialog.busy && !name.is_empty() && (!dialog.nest || dialog.parent.is_some())
        })
    }

    fn create_label(&mut self, cx: &mut Context<Self>) {
        if !self.new_label_ready(cx) {
            return;
        }
        let Some(dialog) = &mut self.new_label else {
            return;
        };
        // The same name again: nothing to change.
        if dialog
            .rename
            .as_ref()
            .is_some_and(|(_, old)| old == dialog.name.read(cx).text().trim())
        {
            self.close_new_label(cx);
            return;
        }
        let Some(dialog) = &mut self.new_label else {
            return;
        };
        dialog.busy = true;
        dialog.error = None;
        let account = dialog.account.0;
        let name = dialog.name.read(cx).text().trim().to_owned();
        let parent = dialog.parent.filter(|_| dialog.nest).map(|f| f.0);
        let gmail = dialog.gmail;
        let rename = dialog.rename.as_ref().map(|(folder, _)| *folder);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let created = name.clone();
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    match rename {
                        Some(folder) => daemon::rename_folder(&connection, folder.0, &created)
                            .await
                            .map(|()| folder.0),
                        None => daemon::create_folder(&connection, account, &created, parent).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    if let Some(dialog) = &mut this.new_label {
                        dialog.busy = false;
                    }
                    this.close_new_label(cx);
                    this.refresh(false, cx);
                    let text = match (rename.is_some(), gmail) {
                        (false, true) => tr!("label-created", name = name.as_str()),
                        (false, false) => tr!("label-folder-created", name = name.as_str()),
                        (true, true) => tr!("label-renamed", name = name.as_str()),
                        (true, false) => tr!("label-folder-renamed", name = name.as_str()),
                    };
                    this.show_snackbar(text, None, cx);
                }
                Err(err) => {
                    if let Some(dialog) = &mut this.new_label {
                        dialog.busy = false;
                        dialog.error = Some(format::sentence(&err));
                    }
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    pub(super) fn render_new_label(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let dialog = self.new_label.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.new_label = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let ready = self.new_label_ready(cx);
        let dialog = self.new_label.as_ref()?;
        let gmail = dialog.gmail;
        let renaming = dialog.rename.is_some();
        let focus = dialog.name.focus_handle(cx);
        let focused = focus.is_focused(window);
        let field = div()
            .id("new-label-name")
            .mt(px(if renaming { 20.0 } else { 8.0 }))
            .h(px(44.0))
            .px(px(14.0))
            .flex()
            .items_center()
            .rounded(px(8.0))
            .border_px(2.0)
            .border_color(rgba(if focused {
                th.accent
            } else {
                fade(th.text_faint, 0.8)
            }))
            .text_size(px(15.0))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(div().flex_1().child(dialog.name.clone()));
        let nest = !dialog.parents.is_empty();
        let checkbox = div()
            .id("new-label-nest")
            .mt(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(dialog) = &mut this.new_label {
                    dialog.nest = !dialog.nest;
                    if dialog.nest && dialog.parent.is_none() {
                        dialog.parent = dialog.parents.first().map(|(id, _)| *id);
                    }
                }
                cx.notify();
            }))
            .child(crate::widgets::checkbox(
                "label-nest-box",
                crate::widgets::Check::from(dialog.nest),
                th,
            ))
            .child(if gmail {
                tr!("label-nest")
            } else {
                tr!("label-folder-nest")
            });
        let parents = dialog.nest.then(|| {
            div()
                .id("new-label-parents")
                .mt(px(8.0))
                .ml(px(32.0))
                .max_h(px(200.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(fade(th.text_faint, 0.5)))
                .py(px(4.0))
                .children(dialog.parents.iter().enumerate().map(|(ix, (id, path))| {
                    let id = *id;
                    let on = dialog.parent == Some(id);
                    div()
                        .id(("new-label-parent", ix))
                        // A scrolling column shrinks its children otherwise.
                        .flex_none()
                        .h(px(36.0))
                        .px(px(12.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .text_size(px(14.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(dialog) = &mut this.new_label {
                                dialog.parent = Some(id);
                            }
                            cx.notify();
                        }))
                        .child(radio(if on { 1.0 } else { 0.0 }, th))
                        .child(div().flex_1().min_w_0().truncate().child(path.clone()))
                }))
        });
        let error = dialog.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(self.copyable(err, th))
        });
        let busy = dialog.busy;
        let body =
            div()
                .id("new-label-body")
                .flex()
                .flex_col()
                .overflow_y_scroll()
                .px(px(24.0))
                .pt(px(24.0))
                .pb(px(20.0))
                .child(div().text_size(px(22.0)).line_height(px(30.0)).child(
                    match (renaming, gmail) {
                        (false, true) => tr!("label-new-title"),
                        (false, false) => tr!("label-folder-new-title"),
                        (true, true) => tr!("label-rename-title"),
                        (true, false) => tr!("label-folder-rename-title"),
                    },
                ))
                .when(!renaming, |d| {
                    d.child(
                        div()
                            .mt(px(20.0))
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(if gmail {
                                tr!("label-prompt")
                            } else {
                                tr!("label-folder-prompt")
                            }),
                    )
                })
                .child(field)
                .when(nest, |d| d.child(checkbox))
                .children(parents)
                .children(error)
                .child(
                    div()
                        .mt(px(24.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(div().flex_1())
                        .child(
                            div()
                                .id("new-label-cancel")
                                .h(px(36.0))
                                .px(px(16.0))
                                .flex()
                                .items_center()
                                .rounded_full()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.accent))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
                                .on_click(cx.listener(|this, _, _, cx| this.close_new_label(cx)))
                                .child(tr!("label-cancel")),
                        )
                        .child(
                            filled_button(
                                "new-label-create",
                                match (renaming, busy) {
                                    (false, true) => tr!("label-creating"),
                                    (false, false) => tr!("label-create"),
                                    (true, true) => tr!("label-renaming"),
                                    (true, false) => tr!("label-rename"),
                                },
                                th,
                            )
                            .focus_ring_filled(th)
                            .when(!ready, |d| d.opacity(0.45).cursor_default())
                            .on_click(cx.listener(|this, _, _, cx| this.create_label(cx))),
                        ),
                );
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let card = div()
            .id("new-label")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(200.0)))
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
                .child(
                    div()
                        .id("new-label-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_new_label(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}
