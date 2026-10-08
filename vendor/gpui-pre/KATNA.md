# gpui-pre, patched for Katna

This is `gpui-pre` 0.3.6 from crates.io (Zed's `gpui` at `bcf6582`,
Apache-2.0, see `LICENSE-APACHE`), used through `[patch.crates-io]` in the
workspace `Cargo.toml`. Its examples and integration test are left out,
except the one picture a unit test reads (`examples/image/`).

The first commit that added this directory holds the crate unchanged, so
`git diff` against it shows the whole patch. When GPUI is upgraded, copy the
new version here and apply the same changes, or drop a patch once upstream
GPUI can do it.
