# gpui-pre-windows, patched for Katna

This is `gpui-pre-windows` 0.3.6 from crates.io (Zed's `gpui_windows` at
`bcf6582`, Apache-2.0, see `LICENSE-APACHE`), used through
`[patch.crates-io]` in the workspace `Cargo.toml`.

Katna's change adds the backdrop blur of Katna's wgpu renderer
(`vendor/gpui-pre-wgpu/KATNA.md`) to GPUI's Direct3D 11 renderer, so
frosted menus, popovers, dialogs and panes look the same on Windows as
on Linux (`src/backdrop_blur.rs`, `src/backdrop_blur.hlsl`):

- The same markers: a quad whose border color is
  `backdrop_blur_marker(radius)` is drawn over a blur of what is already
  in the frame under it, and one whose border color is `erase_marker()`
  clears what is under it, as far as it covers each pixel. Both are
  exported, with `backdrop_blur_supported()`, for `katna_ui::frost`.
- `directx_renderer.rs`: in a batch of quads (`draw_quad_batch`), at a
  marked quad the frame region under it is copied from the back buffer
  (`CopySubresourceRegion`), blurred (dual Kawase: halved a few times,
  then grown back, the same passes and weights as the wgpu shader), drawn
  inside the quad's rounded, clipped shape, and the frame is bound again
  for the rest of the scene. The blur's parameters use constant buffer
  slot 2, so GPUI's own slots 0 and 1 stay bound. An erasing quad is drawn
  with a blend state that keeps the destination times one minus the
  quad's coverage.
- The blur's shaders are compiled like GPUI's: by `fxc` in `build.rs` for
  release builds, from the source file at run time in debug builds. If
  they cannot be made, `backdrop_blur_supported()` is false and marked
  quads are drawn as plain quads.

It also changes drop shadows (`shaders.hlsl`, `shadow_fragment`): a
shadow is drawn only outside its element, as in CSS, so it does not
darken a translucent element. Upstream draws it under the whole element,
which an opaque element hides.

The first commit that added this directory holds the crate unchanged, so
`git diff` against it shows the whole patch. When GPUI is upgraded, copy
the new version here and apply the same change, or drop the patch once
upstream GPUI can do this.
