// SPDX-License-Identifier: GPL-3.0-or-later

//! Finding a page of Settings: one list beside the open page, sorted by
//! scope. "All apps" holds what every Katna app shares; "Apps" holds each
//! app's own pages, Mail's folding open under it. On a phone the list fills
//! the page until a page is picked.

use gpui::{AnyElement, Context, FontWeight, Stateful, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{space, text};

use super::{Scope, Section};
use crate::theme::Theme;
use crate::widgets::{FocusRing, fold_arrow, fold_box, icon};
use crate::window::MailWindow;

/// The side list's width beside the open page.
pub(in crate::window) const NAV_WIDTH: f32 = 232.0;
/// A page's icon in the side list.
const NAV_ICON: f32 = 18.0;
/// How far a page of an app (Mail's Reading) sits in from its app: past
/// the app's icon, so the names line up.
const SUB_INDENT: f32 = NAV_ICON + space::S4;

impl MailWindow {
    /// Mail's row: folds its pages open or closed. Opening it on another
    /// app's page also opens Mail's first page.
    fn toggle_settings_mail(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(page) = &mut self.settings_page else {
            return;
        };
        page.mail_open = !page.mail_open;
        page.mail_fold.turn();
        let open = page.mail_open;
        let elsewhere = page.section.scope() != Scope::Mail;
        if open && elsewhere && !self.layout.shape.is_phone() {
            self.page_section(Scope::Mail.first(), window, cx);
        }
        cx.notify();
    }

    /// The page's back arrow: on a phone, from a page back to the list of
    /// pages; otherwise out of Settings.
    pub(in crate::window) fn settings_back(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.layout.shape.is_phone()
            && let Some(page) = &mut self.settings_page
            && !page.list
            && page.query.is_empty()
        {
            page.list = true;
            cx.notify();
            return;
        }
        self.close_settings_page(window, cx);
    }

    /// The list of pages, `section` picked: beside the page, or the whole
    /// page on a phone (`phone`), where nothing is picked and the rows are
    /// taller.
    pub(in crate::window) fn settings_nav(
        &self,
        section: Section,
        phone: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(page) = &self.settings_page else {
            return div().into_any_element();
        };
        let focus = page.focus.clone();
        let mail_open = page.mail_open;
        let row = |s: Section, sub: bool, cx: &mut Context<Self>| -> Stateful<gpui::Div> {
            let on = !phone && s == section;
            let color = if on {
                th.row_selected_text
            } else {
                th.text_dim
            };
            crate::widgets::pane_row(("settings-nav", s as usize), on, th)
                .when(on, |d| {
                    d.track_focus(&focus).font_weight(FontWeight::MEDIUM)
                })
                .focus_ring(th)
                .when(phone, |d| d.min_h(px(48.0)))
                // A page still to come is fainter until picked.
                .when(super::super::settings_search::is_coming(s) && !on, |d| {
                    d.text_color(rgba(th.text_faint))
                })
                .on_click(cx.listener(move |this, _, window, cx| this.page_section(s, window, cx)))
                .map(|d| {
                    if sub {
                        d.pl(px(space::S3 + SUB_INDENT))
                    } else {
                        d.child(icon(s.icon(), color, NAV_ICON))
                    }
                })
                .child(div().flex_1().min_w_0().truncate().child(s.label()))
                .when(phone, |d| {
                    d.child(icon("chevron-right", th.text_faint, NAV_ICON))
                })
        };
        let heading = |text: String| {
            div()
                .px(px(space::S3))
                .pb(px(space::S2))
                .text_size(px(text::CAPTION))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .child(text)
        };
        let shared: Vec<_> = Scope::Katna
            .sections()
            .filter(|s| !s.at_end())
            .map(|s| row(s, false, cx))
            .collect();
        let end: Vec<_> = Scope::Katna
            .sections()
            .filter(|s| s.at_end())
            .map(|s| row(s, false, cx))
            .collect();
        let mail_pages: Vec<_> = if mail_open {
            Scope::Mail.sections().map(|s| row(s, true, cx)).collect()
        } else {
            Vec::new()
        };
        let mail = crate::widgets::pane_row("settings-nav-mail", false, th)
            .focus_ring(th)
            .when(phone, |d| d.min_h(px(48.0)))
            .on_click(cx.listener(|this, _, window, cx| this.toggle_settings_mail(window, cx)))
            .child(icon(Scope::Mail.icon(), th.text_dim, NAV_ICON))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .child(Scope::Mail.label()),
            )
            .child(fold_arrow(
                "settings-nav-mail-arrow",
                &page.mail_fold,
                mail_open,
                th.text_faint,
                NAV_ICON,
            ));
        // Apps turned off leave the list; Settings > Apps turns them on.
        let apps: Vec<_> = [
            Scope::Calendar,
            Scope::Contacts,
            Scope::Tasks,
            Scope::Notes,
            Scope::Files,
        ]
        .into_iter()
        .filter(|scope| scope.app().is_none_or(|app| self.config.app_on(app)))
        .map(|scope| row(scope.first(), false, cx))
        .collect();
        let overview = row(Section::Apps, false, cx);
        div()
            .flex()
            .flex_col()
            .gap(px(space::S1))
            .child(heading(tr!("settings-group-all-apps")))
            .children(shared)
            .child(heading(tr!("settings-group-apps")).pt(px(space::S4)))
            .child(overview)
            .child(mail)
            .child(fold_box(
                "settings-nav-mail-pages",
                &page.mail_fold,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(space::S1))
                    .children(mail_pages),
            ))
            .children(apps)
            .child(
                div()
                    .my(px(space::S3))
                    .mx(px(space::S3))
                    .h(px(1.0))
                    .bg(rgba(th.divider)),
            )
            .children(end)
            .into_any_element()
    }
}
