// Backdrop blur (added for Katna; not in upstream GPUI). A dual Kawase
// blur: the frame region under a marked quad is copied, halved a few times
// and grown back, and the result is drawn inside the quad's rounded shape.

struct BlurParams {
    // One source texel, in UV units.
    texel: vec2<f32>,
    // The frame's size in device pixels.
    viewport: vec2<f32>,
    // The quad: origin and size in device pixels.
    bounds: vec4<f32>,
    // The quad's content mask: origin and size in device pixels.
    clip: vec4<f32>,
    // The frame region the source texture holds: origin and size.
    region: vec4<f32>,
    // Corner radii: top left, top right, bottom right, bottom left.
    radii: vec4<f32>,
}

@group(0) @binding(0) var<uniform> params: BlurParams;
@group(0) @binding(1) var source: texture_2d<f32>;
@group(0) @binding(2) var source_sampler: sampler;

struct BlurVertex {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// A strip of four vertices covering the whole target.
@vertex
fn vs_blur_full(@builtin(vertex_index) index: u32) -> BlurVertex {
    let corner = vec2<f32>(f32(index & 1u), f32(index >> 1u));
    var out: BlurVertex;
    out.position = vec4<f32>(corner.x * 2.0 - 1.0, 1.0 - corner.y * 2.0, 0.0, 1.0);
    out.uv = corner;
    return out;
}

@fragment
fn fs_blur_down(input: BlurVertex) -> @location(0) vec4<f32> {
    let h = params.texel;
    var sum = textureSample(source, source_sampler, input.uv) * 4.0;
    sum += textureSample(source, source_sampler, input.uv - h);
    sum += textureSample(source, source_sampler, input.uv + h);
    sum += textureSample(source, source_sampler, input.uv + vec2<f32>(h.x, -h.y));
    sum += textureSample(source, source_sampler, input.uv - vec2<f32>(h.x, -h.y));
    return sum / 8.0;
}

fn blur_up(uv: vec2<f32>) -> vec4<f32> {
    let h = params.texel;
    var sum = textureSample(source, source_sampler, uv + vec2<f32>(-h.x * 2.0, 0.0));
    sum += textureSample(source, source_sampler, uv + vec2<f32>(-h.x, h.y)) * 2.0;
    sum += textureSample(source, source_sampler, uv + vec2<f32>(0.0, h.y * 2.0));
    sum += textureSample(source, source_sampler, uv + vec2<f32>(h.x, h.y)) * 2.0;
    sum += textureSample(source, source_sampler, uv + vec2<f32>(h.x * 2.0, 0.0));
    sum += textureSample(source, source_sampler, uv + vec2<f32>(h.x, -h.y)) * 2.0;
    sum += textureSample(source, source_sampler, uv + vec2<f32>(0.0, -h.y * 2.0));
    sum += textureSample(source, source_sampler, uv + vec2<f32>(-h.x, -h.y)) * 2.0;
    return sum / 12.0;
}

@fragment
fn fs_blur_up(input: BlurVertex) -> @location(0) vec4<f32> {
    return blur_up(input.uv);
}

// A strip of four vertices covering the quad.
@vertex
fn vs_blur_composite(@builtin(vertex_index) index: u32) -> BlurVertex {
    let corner = vec2<f32>(f32(index & 1u), f32(index >> 1u));
    let pixel = params.bounds.xy + corner * params.bounds.zw;
    let ndc = pixel / params.viewport * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
    var out: BlurVertex;
    out.position = vec4<f32>(ndc, 0.0, 1.0);
    out.uv = corner;
    return out;
}

// Signed distance from `point` to the rounded rectangle; negative inside.
fn rounded_rect_distance(point: vec2<f32>) -> f32 {
    let half_size = params.bounds.zw * 0.5;
    let center = params.bounds.xy + half_size;
    let p = point - center;
    var radius: f32;
    if (p.x < 0.0) {
        if (p.y < 0.0) {
            radius = params.radii.x;
        } else {
            radius = params.radii.w;
        }
    } else {
        if (p.y < 0.0) {
            radius = params.radii.y;
        } else {
            radius = params.radii.z;
        }
    }
    let q = abs(p) - half_size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_blur_composite(input: BlurVertex) -> @location(0) vec4<f32> {
    let point = input.position.xy;
    let clip_min = params.clip.xy;
    let clip_max = params.clip.xy + params.clip.zw;
    if (any(point < clip_min) || any(point >= clip_max)) {
        discard;
    }
    // Pixels whose centre is inside the shape; the quad drawn over this
    // anti-aliases the edge.
    if (rounded_rect_distance(point) > 0.0) {
        discard;
    }
    let uv = (point - params.region.xy) / params.region.zw;
    return blur_up(uv);
}
