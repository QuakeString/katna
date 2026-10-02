// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Appearance > Colors > Yours: the color schemes people make.
//! Customize copies the selected scheme into a new one and opens it in the
//! editor; Import reads a Katna or KDE (`.colors`) file. The editor shows
//! the eight colors of the light and the dark side next to each other,
//! each with a hex field and a color picker (`super::scheme_color`), the
//! mail window drawn in them, and
//! what reads badly. A card's right-click menu edits, duplicates, exports
//! or deletes it, with Undo. The files are `crate::user_schemes`'.

use std::path::PathBuf;

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, FontWeight, PathPromptOptions, SharedString,
    Subscription, Window, canvas, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_platform::colors::{DesktopScheme, Scheme};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::MailWindow;
use super::context_menu::Rows;
use super::scheme_color::Target;
use super::scheme_picker::{intern, scheme_picture};
use super::settings::Change;
use crate::daemon::Command;
use crate::schemes::{self, SideScheme};
use crate::theme::{Accent, Theme, fade};
use crate::user_schemes::{self, Seed};
use crate::widgets::{elevation, filled_button, icon};

const DIALOG_WIDTH: f32 = 760.0;
const SWATCH: f32 = 22.0;
/// The open editor.
pub(super) struct SchemeEditor {
    /// The scheme, its id `user:<file stem>`; a new one's stem comes from
    /// its name when it is saved.
    scheme: DesktopScheme,
    /// It has a file already.
    saved: bool,
    name: Entity<TextInput>,
    /// The hex fields, light side first, in [`Seed::ALL`]'s order.
    fields: [Vec<Entity<TextInput>>; 2],
    error: Option<String>,
    closing: bool,
    shown: Spring,
    _subscriptions: Vec<Subscription>,
}

pub(super) fn hex(color: u32) -> String {
    format!("#{:06x}", color >> 8)
}

impl SchemeEditor {
    pub(super) fn side(&self, dark: bool) -> Option<&Scheme> {
        if dark {
            self.scheme.dark.as_ref()
        } else {
            self.scheme.light.as_ref()
        }
    }

    fn side_mut(&mut self, dark: bool) -> &mut Option<Scheme> {
        if dark {
            &mut self.scheme.dark
        } else {
            &mut self.scheme.light
        }
    }
}

impl MailWindow {
    /// The scheme `id` as one of your own: its light and dark side as
    /// drawn now, in the accent picked.
    fn scheme_copy(&self, id: &str) -> DesktopScheme {
        let system = &self.desktop_colors.colors;
        let accent = match Accent::parse(&self.config.mail.accent) {
            Accent::Scheme => None,
            Accent::System => system.accent,
            Accent::Color(color) => Some(color),
        };
        let side = |dark: bool| {
            let mut scheme = if let Some(built_in) = schemes::built_in(id) {
                built_in.side(dark).scheme(built_in.id)
            } else if let Some(scheme) = system.scheme(id) {
                match if dark { &scheme.dark } else { &scheme.light } {
                    Some(side) => side.clone(),
                    None => return None,
                }
            } else if let Some(scheme) = (id == schemes::SYSTEM)
                .then(|| system.scheme_for(dark))
                .flatten()
            {
                scheme
            } else {
                katna_side(dark)
            };
            if let Some(accent) = accent {
                scheme.accent = accent | 0xff;
            }
            // The text on the accent as it is drawn, made readable.
            scheme.accent_fg = Theme::from_scheme(&scheme).on_accent | 0xff;
            Some(scheme)
        };
        let name = match schemes::built_in(id) {
            Some(built_in) => tr!(built_in.name),
            None => match system.scheme(id) {
                Some(scheme) => scheme.name.clone(),
                None if id == schemes::SYSTEM => tr!("settings-appearance-colors-system"),
                None => tr!("scheme-katna"),
            },
        };
        DesktopScheme {
            id: String::new(),
            name: tr!("scheme-copy-name", name = name),
            light: side(false),
            dark: side(true),
        }
    }

    /// Customize: the selected scheme copied into the editor.
    pub(super) fn customize_scheme(
        &mut self,
        id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let scheme = self.scheme_copy(id);
        self.open_scheme_editor(scheme, false, window, cx);
    }

    /// Opens your scheme `id` in the editor.
    pub(super) fn edit_scheme(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(scheme) = self
            .desktop_colors
            .user
            .iter()
            .find(|s| s.id == id)
            .cloned()
        {
            self.open_scheme_editor(scheme, true, window, cx);
        }
    }

    fn open_scheme_editor(
        &mut self,
        scheme: DesktopScheme,
        saved: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_menu = None;
        let accent = rgba(self.theme(window).accent).into();
        let name = cx.new(|cx| {
            let mut input = TextInput::new(tr!("scheme-editor-name"), cx);
            input.set_accent(accent);
            input.set_text(scheme.name.clone(), cx);
            input
        });
        let mut subscriptions = vec![cx.subscribe_in(
            &name,
            window,
            |this, _, event: &InputEvent, _, cx| match event {
                InputEvent::Submit => this.save_scheme_editor(cx),
                InputEvent::Cancel => this.close_scheme_editor(cx),
                InputEvent::Changed => {
                    if let Some(editor) = &mut this.scheme_editor {
                        editor.error = None;
                    }
                    cx.notify();
                }
            },
        )];
        let mut fields: [Vec<Entity<TextInput>>; 2] = [Vec::new(), Vec::new()];
        for dark in [false, true] {
            let side = if dark { &scheme.dark } else { &scheme.light };
            for seed in Seed::ALL {
                let text = side.as_ref().map(|s| hex(seed.get(s))).unwrap_or_default();
                let field = cx.new(|cx| {
                    let mut input = TextInput::new("#000000", cx);
                    input.set_accent(accent);
                    input.set_text(text, cx);
                    input
                });
                subscriptions.push(cx.subscribe_in(
                    &field,
                    window,
                    move |this, field, event: &InputEvent, _, cx| match event {
                        InputEvent::Changed => {
                            let text = field.read(cx).text().to_owned();
                            if let Some(color) = katna_platform::colors::parse_css_color(&text)
                                && let Some(editor) = &mut this.scheme_editor
                                && let Some(side) = editor.side_mut(dark)
                            {
                                seed.set(side, color);
                                this.sync_color_picker(Target::Seed(dark, seed), color | 0xff, cx);
                                cx.notify();
                            }
                        }
                        InputEvent::Submit => this.save_scheme_editor(cx),
                        InputEvent::Cancel => this.close_scheme_editor(cx),
                    },
                ));
                fields[usize::from(dark)].push(field);
            }
        }
        window.focus(&name.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.scheme_editor = Some(SchemeEditor {
            scheme,
            saved,
            name,
            fields,
            error: None,
            closing: false,
            shown,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    fn close_scheme_editor(&mut self, cx: &mut Context<Self>) {
        if let Some(editor) = &mut self.scheme_editor {
            editor.closing = true;
            editor.shown.set(0.0);
        }
        cx.notify();
    }

    /// Sets `seed` of a side to `color`, in its hex field too.
    pub(super) fn set_scheme_seed(
        &mut self,
        dark: bool,
        seed: Seed,
        color: u32,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = &mut self.scheme_editor else {
            return;
        };
        if let Some(side) = editor.side_mut(dark) {
            seed.set(side, color);
        }
        let ix = Seed::ALL.iter().position(|s| *s == seed).unwrap_or(0);
        let field = editor.fields[usize::from(dark)][ix].clone();
        field.update(cx, |field, cx| field.set_text(hex(color), cx));
        cx.notify();
    }

    /// Gives the scheme the side it lacks: a dark side worked out from the
    /// light one, or Katna's light side.
    fn add_scheme_side(&mut self, dark: bool, cx: &mut Context<Self>) {
        let Some(editor) = &mut self.scheme_editor else {
            return;
        };
        let side = match (dark, &editor.scheme.light) {
            (true, Some(light)) => user_schemes::dark_from_light(light),
            _ => katna_side(dark),
        };
        *editor.side_mut(dark) = Some(side.clone());
        for (ix, seed) in Seed::ALL.into_iter().enumerate() {
            let field = editor.fields[usize::from(dark)][ix].clone();
            let text = hex(seed.get(&side));
            field.update(cx, |field, cx| field.set_text(text, cx));
        }
        cx.notify();
    }

    fn remove_scheme_side(&mut self, dark: bool, cx: &mut Context<Self>) {
        if let Some(editor) = &mut self.scheme_editor
            && editor.side(!dark).is_some()
        {
            *editor.side_mut(dark) = None;
            self.drop_color_picker(|t| matches!(t, Target::Seed(d, _) if d == dark));
            cx.notify();
        }
    }

    fn save_scheme_editor(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = &self.scheme_editor else {
            return;
        };
        let name = editor.name.read(cx).text().trim().to_owned();
        if name.is_empty() {
            return;
        }
        let mut scheme = editor.scheme.clone();
        scheme.name = name.clone();
        for side in [&mut scheme.light, &mut scheme.dark].into_iter().flatten() {
            side.name = name.clone();
        }
        if !editor.saved {
            let stem = user_schemes::new_stem(&name, &self.desktop_colors.user);
            scheme.id = format!("{}{stem}", user_schemes::PREFIX);
        }
        match self.write_user_scheme(&scheme) {
            Ok(()) => {
                self.close_scheme_editor(cx);
                self.apply(Change::Colors(intern(&scheme.id)), cx);
            }
            Err(err) => {
                if let Some(editor) = &mut self.scheme_editor {
                    editor.error = Some(err);
                }
                cx.notify();
            }
        }
    }

    fn user_schemes_dir(&self) -> PathBuf {
        user_schemes::dir(self.paths.config_dir())
    }

    /// Saves `scheme` and lists it.
    fn write_user_scheme(&mut self, scheme: &DesktopScheme) -> Result<(), String> {
        let dir = self.user_schemes_dir();
        user_schemes::save(&dir, scheme).map_err(|err| err.to_string())?;
        self.reload_user_schemes();
        Ok(())
    }

    fn reload_user_schemes(&mut self) {
        let user = user_schemes::load_all(&self.user_schemes_dir());
        self.desktop_colors.set_user(user);
    }

    /// Duplicate: a copy of your scheme `id` beside it.
    pub(super) fn duplicate_scheme(&mut self, id: &str, cx: &mut Context<Self>) {
        self.context_menu = None;
        let Some(mut scheme) = self
            .desktop_colors
            .user
            .iter()
            .find(|s| s.id == id)
            .cloned()
        else {
            return;
        };
        scheme.name = tr!("scheme-copy-name", name = scheme.name.clone());
        let stem = user_schemes::new_stem(&scheme.name, &self.desktop_colors.user);
        scheme.id = format!("{}{stem}", user_schemes::PREFIX);
        if let Err(err) = self.write_user_scheme(&scheme) {
            self.show_snackbar(err, None, cx);
        }
        cx.notify();
    }

    /// Delete, with Undo: the file goes, and Katna's colors come back if
    /// it was in use.
    pub(super) fn delete_scheme(&mut self, id: &str, cx: &mut Context<Self>) {
        self.context_menu = None;
        if let Some(editor) = &self.scheme_editor
            && editor.scheme.id == id
        {
            self.close_scheme_editor(cx);
        }
        let Some(path) = user_schemes::path(&self.user_schemes_dir(), id) else {
            return;
        };
        let name = self
            .desktop_colors
            .user
            .iter()
            .find(|s| s.id == id)
            .map(|s| s.name.clone())
            .unwrap_or_default();
        let contents = std::fs::read_to_string(&path).unwrap_or_default();
        if let Err(err) = std::fs::remove_file(&path) {
            self.show_snackbar(err.to_string(), None, cx);
            return;
        }
        self.reload_user_schemes();
        let was_used = self.config.mail.colors() == id;
        if was_used {
            self.apply(Change::Colors(schemes::KATNA), cx);
        }
        self.show_snackbar(
            tr!("scheme-deleted", name = name),
            Some(Command::RestoreScheme(id.to_owned(), contents, was_used)),
            cx,
        );
        cx.notify();
    }

    /// Undo of [`MailWindow::delete_scheme`].
    pub(super) fn restore_scheme(
        &mut self,
        id: &str,
        contents: &str,
        was_used: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(path) = user_schemes::path(&self.user_schemes_dir(), id) else {
            return;
        };
        let written = std::fs::create_dir_all(self.user_schemes_dir())
            .and_then(|()| std::fs::write(&path, contents));
        if let Err(err) = written {
            self.show_snackbar(err.to_string(), None, cx);
            return;
        }
        self.reload_user_schemes();
        if was_used {
            self.apply(Change::Colors(intern(id)), cx);
        }
        cx.notify();
    }

    /// Export: your scheme `id` saved where you choose.
    pub(super) fn export_scheme(&mut self, id: &str, cx: &mut Context<Self>) {
        self.context_menu = None;
        let Some(scheme) = self
            .desktop_colors
            .user
            .iter()
            .find(|s| s.id == id)
            .cloned()
        else {
            return;
        };
        let file = format!("{}.toml", id.trim_start_matches(user_schemes::PREFIX));
        let dir = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        let prompt = cx.prompt_for_new_path(&dir, Some(&file));
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(path))) = prompt.await else {
                return;
            };
            let written = std::fs::write(&path, user_schemes::to_toml(&scheme));
            this.update(cx, |this, cx| match written {
                Ok(()) => this.show_snackbar(tr!("scheme-exported", name = scheme.name), None, cx),
                Err(err) => this.show_snackbar(err.to_string(), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Import: a Katna scheme file or a KDE `.colors` file as one of yours,
    /// put in use.
    pub(super) fn import_scheme(&mut self, cx: &mut Context<Self>) {
        let chosen = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr!("scheme-import").into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let read = std::fs::read_to_string(&path);
            this.update(cx, |this, cx| {
                let scheme = read.map_err(|err| err.to_string()).and_then(|text| {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("colors")
                        .to_owned();
                    if path.extension().is_some_and(|e| e == "colors") {
                        Ok(user_schemes::from_kde(&stem, &text))
                    } else {
                        user_schemes::parse(&stem, &text)
                    }
                });
                let mut scheme = match scheme {
                    Ok(scheme) => scheme,
                    Err(err) => {
                        this.show_snackbar(tr!("scheme-import-failed", error = err), None, cx);
                        return;
                    }
                };
                let stem = user_schemes::new_stem(&scheme.name, &this.desktop_colors.user);
                scheme.id = format!("{}{stem}", user_schemes::PREFIX);
                match this.write_user_scheme(&scheme) {
                    Ok(()) => this.apply(Change::Colors(intern(&scheme.id)), cx),
                    Err(err) => this.show_snackbar(err, None, cx),
                }
            })
            .ok();
        })
        .detach();
    }

    /// The right-click menu of scheme `id`'s card: Edit, Duplicate, Export
    /// and Delete for your own, Customize for the others.
    pub(super) fn scheme_menu_rows(
        &self,
        id: &'static str,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Rows {
        let mut rows = Rows::new(rh);
        let item = |key: &'static str, icon: &str, label: String| {
            self.context_item(key, icon, label, rh, th, cx)
        };
        if !id.starts_with(user_schemes::PREFIX) {
            rows.item(
                item("scheme-customize", "tune", tr!("scheme-customize")).on_click(
                    cx.listener(move |this, _, window, cx| this.customize_scheme(id, window, cx)),
                ),
            );
            return rows;
        }
        rows.item(
            item("scheme-edit", "pen", tr!("scheme-edit"))
                .on_click(cx.listener(move |this, _, window, cx| this.edit_scheme(id, window, cx))),
        );
        rows.item(
            item("scheme-duplicate", "copy", tr!("scheme-duplicate"))
                .on_click(cx.listener(move |this, _, _, cx| this.duplicate_scheme(id, cx))),
        );
        rows.item(
            item("scheme-export", "download", tr!("scheme-export"))
                .on_click(cx.listener(move |this, _, _, cx| this.export_scheme(id, cx))),
        );
        rows.rule(th);
        rows.item(
            item("scheme-delete", "trash", tr!("scheme-delete"))
                .on_click(cx.listener(move |this, _, _, cx| this.delete_scheme(id, cx))),
        );
        rows
    }

    pub(super) fn render_scheme_editor(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let th = &th.lifted();
        let editor = self.scheme_editor.as_mut()?;
        let t = editor.shown.tick(window, reduce);
        if editor.closing && editor.shown.settled() {
            self.scheme_editor = None;
            self.drop_color_picker(|t| matches!(t, Target::Seed(..)));
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let editor = self.scheme_editor.as_ref()?;
        let ready = !editor.name.read(cx).text().trim().is_empty();
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let narrow = vw < DIALOG_WIDTH + 32.0;
        let name_focus = editor.name.focus_handle(cx);
        let name_focused = name_focus.is_focused(window);
        let title = if editor.saved {
            tr!("scheme-editor-edit")
        } else {
            tr!("scheme-editor-new")
        };
        let sides = div()
            .mt(px(16.0))
            .flex()
            .when(narrow, |d| d.flex_col())
            .when(!narrow, |d| d.flex_row())
            .gap(px(16.0))
            .child(self.scheme_editor_side(false, th, window, cx))
            .child(self.scheme_editor_side(true, th, window, cx));
        let error = editor.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(err)
        });
        let body = div()
            .id("scheme-editor-body")
            .flex()
            .flex_col()
            .max_h(px((vh - 48.0).max(200.0)))
            .overflow_y_scroll()
            .px(px(24.0))
            .pt(px(24.0))
            .pb(px(20.0))
            .child(div().text_size(px(22.0)).line_height(px(30.0)).child(title))
            .child(
                div()
                    .id("scheme-editor-name")
                    .mt(px(16.0))
                    .h(px(44.0))
                    .px(px(14.0))
                    .flex()
                    .items_center()
                    .rounded(px(8.0))
                    .border_2()
                    .border_color(rgba(if name_focused {
                        th.accent
                    } else {
                        fade(th.text_faint, 0.8)
                    }))
                    .text_size(px(15.0))
                    .cursor_text()
                    .on_click(move |_, window, cx| window.focus(&name_focus, cx))
                    .child(div().flex_1().child(editor.name.clone())),
            )
            .child(sides)
            .children(error)
            .child(
                div()
                    .mt(px(20.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .when(editor.saved, |d| {
                        let id = editor.scheme.id.clone();
                        d.child(
                            text_button("scheme-editor-delete", tr!("scheme-delete"), th.error)
                                .on_click(
                                    cx.listener(move |this, _, _, cx| this.delete_scheme(&id, cx)),
                                ),
                        )
                    })
                    .child(div().flex_1())
                    .child(
                        text_button(
                            "scheme-editor-cancel",
                            tr!("contacts-edit-cancel"),
                            th.accent,
                        )
                        .on_click(cx.listener(|this, _, _, cx| this.close_scheme_editor(cx))),
                    )
                    .child(
                        filled_button("scheme-editor-save", tr!("contacts-edit-save"), th)
                            .when(!ready, |d| d.opacity(0.45).cursor_default())
                            .on_click(cx.listener(|this, _, _, cx| this.save_scheme_editor(cx))),
                    ),
            );
        let card = div()
            .id("scheme-editor")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(DIALOG_WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(super::PANEL_RADIUS))
            .map(|d| crate::widgets::frosted(d, th, th.surface, super::PANEL_RADIUS))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .child(body);
        let picker = self.render_color_picker(|t| matches!(t, Target::Seed(..)), th, window, cx);
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
                        .id("scheme-editor-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_scheme_editor(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .children(picker)
                .into_any_element(),
        )
    }

    /// One side of the editor: its picture, its eight colors, what reads
    /// badly, or a button to add it.
    fn scheme_editor_side(
        &self,
        dark: bool,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let editor = self.scheme_editor.as_ref().expect("the editor is open");
        let heading = div()
            .flex()
            .flex_row()
            .items_center()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(div().flex_1().child(if dark {
                tr!("scheme-editor-dark")
            } else {
                tr!("scheme-editor-light")
            }));
        let column = div().flex_1().min_w_0().flex().flex_col().gap(px(4.0));
        let Some(side) = editor.side(dark) else {
            let label = if dark && editor.scheme.light.is_some() {
                tr!("scheme-editor-make-dark")
            } else if dark {
                tr!("scheme-editor-add-dark")
            } else {
                tr!("scheme-editor-add-light")
            };
            return column.child(heading).child(
                div()
                    .mt(px(8.0))
                    .h(px(120.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(10.0))
                    .border_1()
                    .border_dashed()
                    .border_color(rgba(th.divider))
                    .child(
                        text_button(
                            if dark {
                                "scheme-add-dark"
                            } else {
                                "scheme-add-light"
                            },
                            label,
                            th.accent,
                        )
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.add_scheme_side(dark, cx)),
                        ),
                    ),
            );
        };
        let picture = div()
            .h(px(84.0))
            .flex()
            .rounded(px(10.0))
            .overflow_hidden()
            .border_1()
            .border_color(rgba(th.outline))
            // Rounded itself: GPUI clips children to a rectangle.
            .child(scheme_picture(&Theme::from_scheme(side)).rounded(px(9.0)));
        let one_side = editor.side(!dark).is_none();
        let heading = heading.when(!one_side, |d| {
            d.child(
                div()
                    .id(if dark {
                        "scheme-remove-dark"
                    } else {
                        "scheme-remove-light"
                    })
                    .size(px(28.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .tooltip(crate::widgets::tip(tr!("scheme-editor-remove-side"), th))
                    .on_click(cx.listener(move |this, _, _, cx| this.remove_scheme_side(dark, cx)))
                    .child(icon("remove", th.text_dim, 18.0)),
            )
        });
        let mut rows = div().mt(px(6.0)).flex().flex_col();
        for (ix, seed) in Seed::ALL.into_iter().enumerate() {
            let color = seed.get(side);
            let field = editor.fields[usize::from(dark)][ix].clone();
            let focus = field.focus_handle(cx);
            let focused = focus.is_focused(window);
            let target = Target::Seed(dark, seed);
            let open = self
                .color_picker
                .as_ref()
                .is_some_and(|p| p.target() == target);
            let swatches = self.color_swatches.clone();
            rows = rows.child(
                div()
                    .h(px(34.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .id(SharedString::from(format!("seed-{dark}-{ix}")))
                            .size(px(SWATCH))
                            .flex_none()
                            .rounded(px(6.0))
                            .bg(rgba(color))
                            .relative()
                            .border_1()
                            .border_color(rgba(fade(th.text, 0.25)))
                            .when(open, |d| d.border_2().border_color(rgba(th.accent)))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.toggle_color_picker(target, window, cx)
                            }))
                            .child(
                                canvas(
                                    move |bounds, _, _| {
                                        swatches.borrow_mut().insert(target, bounds);
                                    },
                                    |_, _, _, _| {},
                                )
                                .absolute()
                                .size_full(),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(13.0))
                            .child(tr!(seed.name())),
                    )
                    .child(
                        div()
                            .id(SharedString::from(format!("seed-hex-{dark}-{ix}")))
                            .w(px(96.0))
                            .h(px(28.0))
                            .px(px(8.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .bg(rgba(th.chip))
                            .border_1()
                            .border_color(rgba(if focused { th.accent } else { 0x00000000 }))
                            .text_size(px(13.0))
                            .cursor_text()
                            .on_click(move |_, window, cx| window.focus(&focus, cx))
                            .child(div().flex_1().child(field)),
                    ),
            );
        }
        let low = user_schemes::low_contrast(side);
        let check = div()
            .mt(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .text_size(px(12.0))
            .text_color(rgba(if low.is_empty() {
                th.text_faint
            } else {
                th.error
            }))
            .child(icon(
                if low.is_empty() { "check" } else { "warning" },
                if low.is_empty() {
                    th.text_faint
                } else {
                    th.error
                },
                16.0,
            ))
            .child(if low.is_empty() {
                tr!("scheme-editor-readable")
            } else {
                let names: Vec<String> = low.iter().map(|(seed, _)| tr!(seed.name())).collect();
                tr!("scheme-editor-hard-to-read", colors = names.join(", "))
            });
        column
            .child(heading)
            .child(picture)
            .child(rows)
            .child(check)
    }
}

/// Katna's own palette as a scheme side.
fn katna_side(dark: bool) -> Scheme {
    let th = Theme::new(dark);
    let opaque = |color: u32| color | 0xff;
    Scheme {
        name: tr!("scheme-katna"),
        window_bg: opaque(th.page),
        window_fg: opaque(th.text),
        view_bg: opaque(th.surface),
        view_fg: opaque(th.text),
        inactive_fg: opaque(th.text_faint),
        accent: opaque(th.accent),
        accent_fg: opaque(th.on_accent),
        negative: opaque(th.error),
    }
}

/// A plain text button in `color`, as dialogs' Cancel.
fn text_button(id: &'static str, label: String, color: u32) -> gpui::Stateful<Div> {
    div()
        .id(id)
        .h(px(36.0))
        .px(px(16.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(color))
        .cursor_pointer()
        .hover(move |s| s.bg(rgba(fade(color, 0.08))))
        .child(label)
}
