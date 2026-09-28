# gpui-pre-linux, patched for Katna

This is `gpui-pre-linux` 0.3.6 from crates.io (Zed's `gpui_linux` at
`bcf6582`, Apache-2.0, see `LICENSE-APACHE`), used through
`[patch.crates-io]` in the workspace `Cargo.toml`.

Katna's changes add KDE global menu support, which GPUI does not have on
Linux:

- `set_kde_appmenu(service, object_path)` (`src/linux/appmenu.rs`) names the
  app's `com.canonical.dbusmenu` menu bar.
- Wayland: normal windows get an `org_kde_kwin_appmenu` with that address
  (`src/linux/wayland/client.rs` binds the manager, `window.rs` creates it).
- X11: normal windows get the `_KDE_NET_WM_APPMENU_SERVICE_NAME` and
  `_KDE_NET_WM_APPMENU_OBJECT_PATH` properties (`src/linux/x11/window.rs`).

It also adds background blur for translucent windows
(`src/linux/effects.rs`):

- `ext_background_effect_v1` (KWin 6.7 has only this, not
  `org_kde_kwin_blur`), bound in `wayland/client.rs`; its capabilities
  event says whether the compositor can blur. Preferred over
  `org_kde_kwin_blur` in `wayland/window.rs` (`update_blur`).
- X11: `_KDE_NET_WM_BLUR_BEHIND_REGION` on `Blurred` windows
  (`x11/window.rs`), and KWin's blur detected from the root window's
  properties (`x11/client.rs`).
- `compositor_blur()` says whether any of these blurs.
- The blur region is the window's frame, not the whole surface: under
  client-side decorations it leaves out the shadow margin and follows the
  rounded corners (`set_client_corner_radius`). KWin reads the region
  relative to the frame (window geometry), not the surface, so it is given
  that way.
- X11: client-side decorations make the surface transparent (as on
  Wayland), and going back to server-side decorations removes
  `_GTK_FRAME_EXTENTS`, so a window can switch both ways while open.

It also opens the main window where it was (`src/linux/placement.rs`):

- `restore_placement(Placement { session, name, restore })` names the
  next normal window's session and whether to put it back.
- Wayland: `xdg-session-management-v1` (`src/linux/wayland/session.rs`,
  bindings generated from `protocols/xdg-session-management-v1.xml`, which
  `wayland-protocols` ships without Rust code). The window joins the
  session before its first commit; `placement_session()` is the session's
  id to keep.
- X11: a restored window opens exactly at its origin (no 2 px nudge,
  `USPosition` with static gravity), `window_bounds()` reads the origin
  from the server rather than from configure events (which are relative
  to the window manager's frame), and a window that opens maximized sets
  `_NET_WM_STATE` before it is mapped, as window managers ignore requests
  for unmapped windows.

It also carries clipboard and drag-and-drop content beyond plain text
(`src/linux/transfer.rs`, exported from the crate root):

- `read_rich(|| cx.read_from_clipboard())` reads, on Wayland
  (`wayland/clipboard.rs`) and X11 (`x11/clipboard.rs`, one connection,
  TARGETS then each type), the owner's HTML (in the string entry's
  metadata, `clipboard_html`), plain text, one picture and copied files
  (`text/uri-list`, `x-special/gnome-copied-files`) as `ExternalPaths`.
  Outside `read_rich` reads are unchanged. UTF-16 HTML (with a byte order
  mark) and Windows' "HTML Format" header are handled.
- `html_item(plain, html)` offers `text/html` beside the text types
  (Wayland `send` answers by MIME type; X11 `set_item` also stores the text
  under the `text/plain` names TARGETS lists, which upstream advertised
  but could not serve).
- Drops: Wayland accepts the offer on enter with the enter serial (upstream
  accepted `text/uri-list` on the offer event, before any serial of the
  drag) and reads files or content in one go; X11 reads the whole type
  list, asks for each wanted type in a property of its own name, reads
  whole properties (upstream read 4 KB, which cut long file lists) and
  only accepts drags it can take. Files arrive as before; content (HTML,
  text, a picture) arrives as a drop of one made-up path, and
  `dropped_content(paths)` gives it.
- A drag moving over the window is sent as a mouse move with the button
  held (`drag_move`), not `FileDropEvent::Pending`: GPUI notes the input
  kind before it turns Pending into a move, so after typing nothing counted
  as hovered and the drop landed nowhere.

It also raises windows with another app's activation token
(`src/linux/activation.rs`): `set_activation_token(token)` keeps the token
a tray icon or notification passed along, and the next Wayland
`Window::activate` uses it, so the window comes forward (and out of
minimized) instead of only asking for attention.

The first commit that added this directory holds the crate unchanged, so
`git diff` against it shows the whole patch. When GPUI is upgraded, copy the
new version here and apply the same change, or drop the patch once upstream
GPUI can do this.
