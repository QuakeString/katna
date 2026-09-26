# gpui-pre-linux, patched for Katna

This is `gpui-pre-linux` 0.3.6 from crates.io (Zed's `gpui_linux` at
`bcf6582`, Apache-2.0, see `LICENSE-APACHE`), used through
`[patch.crates-io]` in the workspace `Cargo.toml`.

Katna's change adds KDE global menu support, which GPUI does not have on
Linux:

- `set_kde_appmenu(service, object_path)` (`src/linux/appmenu.rs`) names the
  app's `com.canonical.dbusmenu` menu bar.
- Wayland: normal windows get an `org_kde_kwin_appmenu` with that address
  (`src/linux/wayland/client.rs` binds the manager, `window.rs` creates it).
- X11: normal windows get the `_KDE_NET_WM_APPMENU_SERVICE_NAME` and
  `_KDE_NET_WM_APPMENU_OBJECT_PATH` properties (`src/linux/x11/window.rs`).

The first commit that added this directory holds the crate unchanged, so
`git diff` against it shows the whole patch. When GPUI is upgraded, copy the
new version here and apply the same change, or drop the patch once upstream
GPUI can do this.
