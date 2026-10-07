//! Backdrop blur for frosted menus and popovers (added for Katna; not in
//! upstream GPUI). The Direct3D 11 twin of the blur in Katna's wgpu
//! renderer (`vendor/gpui-pre-wgpu/src/backdrop_blur.rs`), with the same
//! markers, so `katna_ui::frost` draws the same frost on Windows as on
//! Linux.
//!
//! GPUI has no backdrop filter. A quad whose border colour is
//! [`backdrop_blur_marker`] asks the renderer to blur what is already drawn
//! under the quad before drawing the quad itself, so a translucent fill
//! looks like frosted glass. The marker's hue is out of range, so no real
//! colour matches it, and the quad has no border, so a renderer without
//! this change draws the quad as a plain fill. Its alpha starts at one and
//! GPUI multiplies it by the element's opacity like any colour, so a
//! panel fading out blurs less and less instead of leaving a blurred
//! ghost of itself behind.
//!
//! At a marked quad the renderer copies the frame region under it, blurs
//! the copy with a dual Kawase blur, draws it inside the quad's rounded,
//! clipped shape and carries on.

use std::{
    slice,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::{Context as _, Result};
use gpui::{Hsla, Quad};
use windows::Win32::Graphics::{
    Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
    Direct3D11::*,
    Dxgi::Common::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
};

use crate::directx_renderer::shader_resources::{RawShaderBytes, ShaderModule, ShaderTarget};

/// The hue that marks a quad; real hues are 0 to 1.
const MARKER_HUE: f32 = -1024.0;
/// The hue that marks a quad that clears what is under it.
const ERASE_HUE: f32 = -2048.0;

static SUPPORTED: AtomicBool = AtomicBool::new(false);

/// The border colour that marks a quad for a backdrop blur of `radius`
/// device pixels. Give the quad no border.
pub fn backdrop_blur_marker(radius: f32) -> Hsla {
    Hsla {
        h: MARKER_HUE,
        s: radius.max(0.0),
        l: 0.0,
        a: 1.0,
    }
}

/// The border colour that marks a quad that clears what is already drawn
/// under it, as far as it covers each pixel, so what is drawn next over a
/// translucent window shows the desktop rather than what was under. Give
/// the quad no border and an opaque fill: a renderer without this change
/// draws that fill.
pub fn erase_marker() -> Hsla {
    Hsla {
        h: ERASE_HUE,
        s: 0.0,
        l: 0.0,
        a: 1.0,
    }
}

/// Whether `quad` clears what is under it ([`erase_marker`]).
pub(crate) fn is_erase(quad: &Quad) -> bool {
    quad.border_color.h == ERASE_HUE
}

/// Whether the renderer blurs behind marked quads. False until a window
/// has been drawn, and where the blur's shaders could not be made.
pub fn backdrop_blur_supported() -> bool {
    SUPPORTED.load(Ordering::Relaxed)
}

/// Below this, in device pixels, a blur is not worth drawing.
const MIN_RADIUS: f32 = 0.5;

/// The blur radius of a marked quad, in device pixels: the marker's,
/// scaled by the opacity of the element that drew it. `None` for an
/// unmarked quad, and for a blur too slight to draw.
pub(crate) fn marker_radius(quad: &Quad) -> Option<f32> {
    let marker = quad.border_color;
    if marker.h != MARKER_HUE {
        return None;
    }
    let radius = marker.s * marker.a.clamp(0.0, 1.0);
    (radius >= MIN_RADIUS).then_some(radius)
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct BlurParams {
    texel: [f32; 2],
    viewport: [f32; 2],
    bounds: [f32; 4],
    clip: [f32; 4],
    region: [f32; 4],
    radii: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<BlurParams>() % 16 == 0);

/// How many times the copy is halved for a blur of `radius` pixels. Each
/// level roughly doubles the reach.
fn levels(radius: f32) -> usize {
    ((radius / 1.5).max(1.0).log2().round() as usize).clamp(1, 5)
}

/// The frame region a blur of the quad reads: the visible part of the quad,
/// in whole pixels inside the frame, as `(x, y, width, height)`. The blur
/// repeats the region's edge pixels rather than reading past them (as CSS
/// `backdrop-filter` does), so a shadow around the quad does not bleed in.
fn region(quad: &Quad, viewport: (u32, u32)) -> Option<(u32, u32, u32, u32)> {
    let visible = quad.bounds.intersect(&quad.content_mask.bounds);
    let left = visible.origin.x.0.floor().max(0.0);
    let top = visible.origin.y.0.floor().max(0.0);
    let right = (visible.origin.x.0 + visible.size.width.0)
        .ceil()
        .min(viewport.0 as f32);
    let bottom = (visible.origin.y.0 + visible.size.height.0)
        .ceil()
        .min(viewport.1 as f32);
    if right - left < 1.0 || bottom - top < 1.0 {
        return None;
    }
    Some((
        left as u32,
        top as u32,
        (right - left) as u32,
        (bottom - top) as u32,
    ))
}

struct Level {
    texture: ID3D11Texture2D,
    view: Option<ID3D11ShaderResourceView>,
    /// None for level 0, which is copied into rather than drawn into.
    target: Option<ID3D11RenderTargetView>,
    width: u32,
    height: u32,
}

/// Textures for one blur of one size, kept across frames.
struct Targets {
    width: u32,
    height: u32,
    /// Level 0 is the copy of the frame; each next level is half the size.
    levels: Vec<Level>,
}

struct Shaders {
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
}

impl Shaders {
    fn new(device: &ID3D11Device, module: ShaderModule) -> Result<Self> {
        let vertex = {
            let raw = RawShaderBytes::new(module, ShaderTarget::Vertex)?;
            let mut shader = None;
            unsafe { device.CreateVertexShader(raw.as_bytes(), None, Some(&mut shader))? };
            shader.context("creating blur vertex shader")?
        };
        let fragment = {
            let raw = RawShaderBytes::new(module, ShaderTarget::Fragment)?;
            let mut shader = None;
            unsafe { device.CreatePixelShader(raw.as_bytes(), None, Some(&mut shader))? };
            shader.context("creating blur fragment shader")?
        };
        Ok(Self { vertex, fragment })
    }
}

pub(crate) struct BackdropBlur {
    down: Shaders,
    up: Shaders,
    composite: Shaders,
    params: ID3D11Buffer,
    sampler: ID3D11SamplerState,
    /// Blending off: the blurred pixels replace what is there, alpha too.
    replace: ID3D11BlendState,
    format: DXGI_FORMAT,
    /// Indexed by the blur's position in the frame.
    targets: Vec<Targets>,
}

/// What the main drawing had bound, to put back after a blur.
pub(crate) struct Frame<'a> {
    pub texture: &'a ID3D11Texture2D,
    pub view: &'a Option<ID3D11RenderTargetView>,
    pub viewport: &'a D3D11_VIEWPORT,
}

impl BackdropBlur {
    /// `None`, with backdrop blur reported unsupported, when the blur's
    /// shaders or buffers cannot be made: marked quads are then drawn as
    /// plain quads.
    pub(crate) fn new(device: &ID3D11Device, format: DXGI_FORMAT) -> Option<Self> {
        let blur = Self::create(device, format)
            .inspect_err(|error| log::error!("backdrop blur unavailable: {error:#}"))
            .ok();
        SUPPORTED.store(blur.is_some(), Ordering::Relaxed);
        blur
    }

    fn create(device: &ID3D11Device, format: DXGI_FORMAT) -> Result<Self> {
        let down = Shaders::new(device, ShaderModule::BlurDown)?;
        let up = Shaders::new(device, ShaderModule::BlurUp)?;
        let composite = Shaders::new(device, ShaderModule::BlurComposite)?;
        let params = unsafe {
            let desc = D3D11_BUFFER_DESC {
                ByteWidth: std::mem::size_of::<BlurParams>() as u32,
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let mut buffer = None;
            device.CreateBuffer(&desc, None, Some(&mut buffer))?;
            buffer.context("creating blur parameters")?
        };
        let sampler = unsafe {
            let desc = D3D11_SAMPLER_DESC {
                Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
                MipLODBias: 0.0,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D11_COMPARISON_ALWAYS,
                BorderColor: [0.0; 4],
                MinLOD: 0.0,
                MaxLOD: D3D11_FLOAT32_MAX,
            };
            let mut sampler = None;
            device.CreateSamplerState(&desc, Some(&mut sampler))?;
            sampler.context("creating blur sampler")?
        };
        let replace = unsafe {
            let mut desc = D3D11_BLEND_DESC::default();
            desc.RenderTarget[0].BlendEnable = false.into();
            desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;
            let mut state = None;
            device.CreateBlendState(&desc, Some(&mut state))?;
            state.context("creating blur blend state")?
        };
        Ok(Self {
            down,
            up,
            composite,
            params,
            sampler,
            replace,
            format,
            targets: Vec::new(),
        })
    }

    /// Blurs the frame under `quad` into the quad's shape, then binds the
    /// frame for drawing again. `index` is this blur's position in the
    /// frame, which picks its cached textures.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn blur(
        &mut self,
        device: &ID3D11Device,
        context: &ID3D11DeviceContext,
        frame: Frame,
        quad: &Quad,
        radius: f32,
        index: usize,
    ) -> Result<()> {
        let viewport = (frame.viewport.Width as u32, frame.viewport.Height as u32);
        let Some((x, y, width, height)) = region(quad, viewport) else {
            return Ok(());
        };
        let levels = levels(radius);
        if self.targets.len() <= index {
            self.targets.resize_with(index + 1, || Targets {
                width: 0,
                height: 0,
                levels: Vec::new(),
            });
        }
        let cached = &self.targets[index];
        if cached.width != width || cached.height != height || cached.levels.len() != levels + 1 {
            self.targets[index] = self.create_targets(device, width, height, levels)?;
        }
        let targets = &self.targets[index];

        let base = BlurParams {
            texel: [0.0, 0.0],
            viewport: [viewport.0 as f32, viewport.1 as f32],
            bounds: [
                quad.bounds.origin.x.0,
                quad.bounds.origin.y.0,
                quad.bounds.size.width.0,
                quad.bounds.size.height.0,
            ],
            clip: [
                quad.content_mask.bounds.origin.x.0,
                quad.content_mask.bounds.origin.y.0,
                quad.content_mask.bounds.size.width.0,
                quad.content_mask.bounds.size.height.0,
            ],
            region: [x as f32, y as f32, width as f32, height as f32],
            radii: [
                quad.corner_radii.top_left.0,
                quad.corner_radii.top_right.0,
                quad.corner_radii.bottom_right.0,
                quad.corner_radii.bottom_left.0,
            ],
        };

        unsafe {
            // The frame stops being a render target while it is copied.
            context.OMSetRenderTargets(None, None);
            let source = D3D11_BOX {
                left: x,
                top: y,
                front: 0,
                right: x + width,
                bottom: y + height,
                back: 1,
            };
            context.CopySubresourceRegion(
                &targets.levels[0].texture,
                0,
                0,
                0,
                0,
                frame.texture,
                0,
                Some(&source),
            );
            context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            context.VSSetConstantBuffers(2, Some(slice::from_ref(&Some(self.params.clone()))));
            context.PSSetConstantBuffers(2, Some(slice::from_ref(&Some(self.params.clone()))));
            context.PSSetSamplers(0, Some(slice::from_ref(&Some(self.sampler.clone()))));
            context.OMSetBlendState(&self.replace, None, 0xFFFFFFFF);
        }

        let texel = |level: &Level| [0.5 / level.width as f32, 0.5 / level.height as f32];
        for level in 0..levels {
            let source = &targets.levels[level];
            let target = &targets.levels[level + 1];
            self.pass(
                context,
                &self.down,
                source,
                target.target.as_ref(),
                target.width,
                target.height,
                BlurParams {
                    texel: texel(source),
                    ..base
                },
            )?;
        }
        for level in (2..=levels).rev() {
            let source = &targets.levels[level];
            let target = &targets.levels[level - 1];
            self.pass(
                context,
                &self.up,
                source,
                target.target.as_ref(),
                target.width,
                target.height,
                BlurParams {
                    texel: texel(source),
                    ..base
                },
            )?;
        }
        unsafe {
            context.OMSetRenderTargets(Some(slice::from_ref(frame.view)), None);
            context.RSSetViewports(Some(slice::from_ref(frame.viewport)));
        }
        let source = &targets.levels[1];
        self.write_params(
            context,
            BlurParams {
                texel: texel(source),
                ..base
            },
        )?;
        unsafe {
            context.PSSetShaderResources(0, Some(slice::from_ref(&source.view)));
            context.VSSetShader(&self.composite.vertex, None);
            context.PSSetShader(&self.composite.fragment, None);
            context.Draw(4, 0);
            // Nothing keeps reading the blur's textures.
            context.PSSetShaderResources(0, Some(&[None]));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn pass(
        &self,
        context: &ID3D11DeviceContext,
        shaders: &Shaders,
        source: &Level,
        target: Option<&ID3D11RenderTargetView>,
        width: u32,
        height: u32,
        params: BlurParams,
    ) -> Result<()> {
        let target = target.context("blur level without a render target")?;
        self.write_params(context, params)?;
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        unsafe {
            // The source may have been the last pass's target.
            context.PSSetShaderResources(0, Some(&[None]));
            context.OMSetRenderTargets(Some(&[Some(target.clone())]), None);
            context.RSSetViewports(Some(&[viewport]));
            context.ClearRenderTargetView(target, &[0.0; 4]);
            context.PSSetShaderResources(0, Some(slice::from_ref(&source.view)));
            context.VSSetShader(&shaders.vertex, None);
            context.PSSetShader(&shaders.fragment, None);
            context.Draw(4, 0);
        }
        Ok(())
    }

    fn write_params(&self, context: &ID3D11DeviceContext, params: BlurParams) -> Result<()> {
        unsafe {
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            context.Map(
                &self.params,
                0,
                D3D11_MAP_WRITE_DISCARD,
                0,
                Some(&mut mapped),
            )?;
            std::ptr::copy_nonoverlapping(&params, mapped.pData as *mut BlurParams, 1);
            context.Unmap(&self.params, 0);
        }
        Ok(())
    }

    fn create_targets(
        &self,
        device: &ID3D11Device,
        width: u32,
        height: u32,
        levels: usize,
    ) -> Result<Targets> {
        let levels = (0..=levels)
            .map(|level| {
                let level_width = (width >> level).max(1);
                let level_height = (height >> level).max(1);
                let bind = if level == 0 {
                    D3D11_BIND_SHADER_RESOURCE.0
                } else {
                    D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0
                };
                let desc = D3D11_TEXTURE2D_DESC {
                    Width: level_width,
                    Height: level_height,
                    MipLevels: 1,
                    ArraySize: 1,
                    Format: self.format,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Usage: D3D11_USAGE_DEFAULT,
                    BindFlags: bind as u32,
                    CPUAccessFlags: 0,
                    MiscFlags: 0,
                };
                unsafe {
                    let mut texture = None;
                    device.CreateTexture2D(&desc, None, Some(&mut texture))?;
                    let texture = texture.context("creating blur texture")?;
                    let mut view = None;
                    device.CreateShaderResourceView(&texture, None, Some(&mut view))?;
                    let target = if level == 0 {
                        None
                    } else {
                        let mut target = None;
                        device.CreateRenderTargetView(&texture, None, Some(&mut target))?;
                        target
                    };
                    Ok(Level {
                        texture,
                        view,
                        target,
                        width: level_width,
                        height: level_height,
                    })
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Targets {
            width,
            height,
            levels,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_grow_with_radius() {
        assert_eq!(levels(1.0), 1);
        assert!(levels(12.0) < levels(48.0));
        assert_eq!(levels(10_000.0), 5);
    }
}
