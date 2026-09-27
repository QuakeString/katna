# gpui-pre-wgpu, patched for Katna

This is `gpui-pre-wgpu` 0.3.6 from crates.io (Zed's `gpui_wgpu` at
`bcf6582`, Apache-2.0, see `LICENSE-APACHE`), used through
`[patch.crates-io]` in the workspace `Cargo.toml`. Its benchmark is left
out (it needs files from Zed's repository).

Katna's change adds a backdrop blur, which GPUI does not have, for frosted
menus and popovers (`src/backdrop_blur.rs`, `src/backdrop_blur.wgsl`):

- A quad whose border color is `backdrop_blur_marker(radius)` (an
  out-of-range hue with zero alpha, so no real color matches it) is drawn
  over a blur of what is already in the frame under it.
- `wgpu_renderer.rs`: the surface is configured with `COPY_SRC` when it
  allows it. At a marked quad, `record_frame` ends the render pass, copies
  the frame region under the quad, blurs it (dual Kawase: halved a few
  times, then grown back), draws it inside the quad's rounded, clipped
  shape, and starts a new pass for the rest of the scene. The region is
  the quad itself; the blur repeats its edge pixels rather than reading
  past them, as CSS `backdrop-filter` does.
- `backdrop_blur_supported()` says whether the blur is drawn. Without
  `COPY_SRC` marked quads are drawn as plain quads.

It also changes drop shadows (`shaders.wgsl`, `fs_shadow`): a shadow is
drawn only outside its element, as in CSS, so it does not darken a
translucent element. Upstream draws it under the whole element, which an
opaque element hides.

`diff -r` against the published crate (in `~/.cargo/registry/src/` once
fetched) shows the whole patch. When GPUI is upgraded, copy the new version
here and apply the same change, or drop the patch once upstream GPUI can do
this.
