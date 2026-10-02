//! Backdrop blur for frosted menus and popovers (added for Katna; not in
//! upstream GPUI).
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
//! The renderer ends its pass at a marked quad, copies the frame region
//! under it, blurs the copy with a dual Kawase blur, draws it inside the
//! quad's rounded, clipped shape and carries on. This needs a surface that can be copied from; without one
//! [`backdrop_blur_supported`] is false and marked quads are drawn plain.

use bytemuck::{Pod, Zeroable};
use gpui::{Hsla, Quad};
use std::sync::atomic::{AtomicBool, Ordering};

/// The hue that marks a quad; real hues are 0 to 1.
const MARKER_HUE: f32 = -1024.0;

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

/// Whether the renderer blurs behind marked quads. False until a window
/// has been drawn, and where the surface cannot be copied from.
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct BlurParams {
    texel: [f32; 2],
    viewport: [f32; 2],
    bounds: [f32; 4],
    clip: [f32; 4],
    region: [f32; 4],
    radii: [f32; 4],
}

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

/// Textures and bindings for one blur of one size, kept across frames.
struct Targets {
    width: u32,
    height: u32,
    levels: usize,
    /// Level 0 is the copy of the frame; each next level is half the size.
    textures: Vec<(wgpu::Texture, wgpu::TextureView)>,
    /// One per pass: the downsamples, the upsamples, then the composite.
    passes: Vec<(wgpu::Buffer, wgpu::BindGroup)>,
}

pub(crate) struct BackdropBlur {
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    down: wgpu::RenderPipeline,
    up: wgpu::RenderPipeline,
    composite: wgpu::RenderPipeline,
    format: wgpu::TextureFormat,
    /// Indexed by the blur's position in the frame.
    targets: Vec<Targets>,
}

impl BackdropBlur {
    /// `None` when the surface cannot be copied from.
    pub(crate) fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        surface_usage: wgpu::TextureUsages,
    ) -> Option<Self> {
        let supported = surface_usage.contains(wgpu::TextureUsages::COPY_SRC);
        SUPPORTED.store(supported, Ordering::Relaxed);
        if !supported {
            return None;
        }
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("backdrop_blur_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("backdrop_blur.wgsl").into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("backdrop_blur_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<BlurParams>() as u64
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("backdrop_blur_pipeline_layout"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipeline = |name: &str, vertex: &str, fragment: &str| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(vertex),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(fragment),
                    // The blurred pixels replace what is there, alpha too.
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let down = pipeline("backdrop_blur_down", "vs_blur_full", "fs_blur_down");
        let up = pipeline("backdrop_blur_up", "vs_blur_full", "fs_blur_up");
        let composite = pipeline(
            "backdrop_blur_composite",
            "vs_blur_composite",
            "fs_blur_composite",
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("backdrop_blur_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Some(Self {
            layout,
            sampler,
            down,
            up,
            composite,
            format,
            targets: Vec::new(),
        })
    }

    /// Blurs the frame under `quad` into the quad's shape. `index` is this
    /// blur's position in the frame, which picks its cached textures.
    /// Records passes on `encoder`; no render pass may be open on it.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn blur(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::Texture,
        frame_view: &wgpu::TextureView,
        viewport: (u32, u32),
        quad: &Quad,
        radius: f32,
        index: usize,
    ) {
        let Some((x, y, width, height)) = region(quad, viewport) else {
            return;
        };
        let levels = levels(radius);
        if self.targets.len() <= index {
            self.targets.resize_with(index + 1, || Targets {
                width: 0,
                height: 0,
                levels: 0,
                textures: Vec::new(),
                passes: Vec::new(),
            });
        }
        let cached = &self.targets[index];
        if cached.width != width || cached.height != height || cached.levels != levels {
            let targets = self.create_targets(device, width, height, levels);
            self.targets[index] = targets;
        }
        let targets = &self.targets[index];

        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: frame,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &targets.textures[0].0,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let size = |level: usize| {
            let t = &targets.textures[level].0;
            (t.width() as f32, t.height() as f32)
        };
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
        // Pass p reads level `source(p)` and writes level `target(p)`.
        let mut pass = 0;
        for level in 0..levels {
            let (w, h) = size(level);
            self.run(
                queue,
                encoder,
                &self.down,
                &targets.passes[pass],
                &targets.textures[level + 1].1,
                BlurParams {
                    texel: [0.5 / w, 0.5 / h],
                    ..base
                },
            );
            pass += 1;
        }
        for level in (2..=levels).rev() {
            let (w, h) = size(level);
            self.run(
                queue,
                encoder,
                &self.up,
                &targets.passes[pass],
                &targets.textures[level - 1].1,
                BlurParams {
                    texel: [0.5 / w, 0.5 / h],
                    ..base
                },
            );
            pass += 1;
        }
        let (w, h) = size(1);
        let (buffer, bind_group) = &targets.passes[pass];
        queue.write_buffer(
            buffer,
            0,
            bytemuck::bytes_of(&BlurParams {
                texel: [0.5 / w, 0.5 / h],
                ..base
            }),
        );
        let mut render = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("backdrop_blur_composite"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: frame_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        render.set_pipeline(&self.composite);
        render.set_bind_group(0, bind_group, &[]);
        render.draw(0..4, 0..1);
    }

    fn run(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        pipeline: &wgpu::RenderPipeline,
        (buffer, bind_group): &(wgpu::Buffer, wgpu::BindGroup),
        target: &wgpu::TextureView,
        params: BlurParams,
    ) {
        queue.write_buffer(buffer, 0, bytemuck::bytes_of(&params));
        let mut render = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("backdrop_blur_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        render.set_pipeline(pipeline);
        render.set_bind_group(0, bind_group, &[]);
        render.draw(0..4, 0..1);
    }

    fn create_targets(
        &self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        levels: usize,
    ) -> Targets {
        let textures: Vec<_> = (0..=levels)
            .map(|level| {
                let usage = if level == 0 {
                    wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING
                } else {
                    wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING
                };
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("backdrop_blur_level"),
                    size: wgpu::Extent3d {
                        width: (width >> level).max(1),
                        height: (height >> level).max(1),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: self.format,
                    usage,
                    view_formats: &[],
                });
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                (texture, view)
            })
            .collect();
        // Downsamples read levels 0..levels, upsamples read levels..2, the
        // composite reads level 1.
        let sources = (0..levels).chain((2..=levels).rev()).chain([1]);
        let passes = sources
            .map(|source| {
                let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("backdrop_blur_params"),
                    size: std::mem::size_of::<BlurParams>() as u64,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("backdrop_blur_bind_group"),
                    layout: &self.layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&textures[source].1),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Sampler(&self.sampler),
                        },
                    ],
                });
                (buffer, bind_group)
            })
            .collect();
        Targets {
            width,
            height,
            levels,
            textures,
            passes,
        }
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
