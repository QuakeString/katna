// Backdrop blur (added for Katna; not in upstream GPUI). A dual Kawase
// blur: the frame region under a marked quad is copied, halved a few times
// and grown back, and the result is drawn inside the quad's rounded shape.
// The same blur as Katna's wgpu renderer (vendor/gpui-pre-wgpu,
// backdrop_blur.wgsl).

cbuffer BlurParams: register(b2) {
    // One source texel, in UV units.
    float2 texel;
    // The frame's size in device pixels.
    float2 viewport;
    // The quad: origin and size in device pixels.
    float4 quad_bounds;
    // The quad's content mask: origin and size in device pixels.
    float4 clip;
    // The frame region the source texture holds: origin and size.
    float4 region;
    // Corner radii: top left, top right, bottom right, bottom left.
    float4 radii;
};

Texture2D<float4> source: register(t0);
SamplerState source_sampler: register(s0);

struct BlurVertex {
    float4 position: SV_Position;
    float2 uv: TEXCOORD0;
};

// A strip of four vertices covering the whole target.
BlurVertex full_vertex(uint index) {
    float2 corner = float2((float)(index & 1u), (float)(index >> 1u));
    BlurVertex output;
    output.position = float4(corner.x * 2.0 - 1.0, 1.0 - corner.y * 2.0, 0.0, 1.0);
    output.uv = corner;
    return output;
}

BlurVertex blur_down_vertex(uint index: SV_VertexID) {
    return full_vertex(index);
}

float4 blur_down_fragment(BlurVertex input): SV_Target {
    float2 h = texel;
    float4 sum = source.Sample(source_sampler, input.uv) * 4.0;
    sum += source.Sample(source_sampler, input.uv - h);
    sum += source.Sample(source_sampler, input.uv + h);
    sum += source.Sample(source_sampler, input.uv + float2(h.x, -h.y));
    sum += source.Sample(source_sampler, input.uv - float2(h.x, -h.y));
    return sum / 8.0;
}

float4 blur_up(float2 uv) {
    float2 h = texel;
    float4 sum = source.Sample(source_sampler, uv + float2(-h.x * 2.0, 0.0));
    sum += source.Sample(source_sampler, uv + float2(-h.x, h.y)) * 2.0;
    sum += source.Sample(source_sampler, uv + float2(0.0, h.y * 2.0));
    sum += source.Sample(source_sampler, uv + float2(h.x, h.y)) * 2.0;
    sum += source.Sample(source_sampler, uv + float2(h.x * 2.0, 0.0));
    sum += source.Sample(source_sampler, uv + float2(h.x, -h.y)) * 2.0;
    sum += source.Sample(source_sampler, uv + float2(0.0, -h.y * 2.0));
    sum += source.Sample(source_sampler, uv + float2(-h.x, -h.y)) * 2.0;
    return sum / 12.0;
}

BlurVertex blur_up_vertex(uint index: SV_VertexID) {
    return full_vertex(index);
}

float4 blur_up_fragment(BlurVertex input): SV_Target {
    return blur_up(input.uv);
}

// A strip of four vertices covering the quad.
BlurVertex blur_composite_vertex(uint index: SV_VertexID) {
    float2 corner = float2((float)(index & 1u), (float)(index >> 1u));
    float2 pixel = quad_bounds.xy + corner * quad_bounds.zw;
    float2 ndc = pixel / viewport * float2(2.0, -2.0) + float2(-1.0, 1.0);
    BlurVertex output;
    output.position = float4(ndc, 0.0, 1.0);
    output.uv = corner;
    return output;
}

// Signed distance from `pt` to the rounded rectangle; negative inside.
float rounded_rect_distance(float2 pt) {
    float2 half_size = quad_bounds.zw * 0.5;
    float2 center = quad_bounds.xy + half_size;
    float2 p = pt - center;
    float radius;
    if (p.x < 0.0) {
        radius = p.y < 0.0 ? radii.x : radii.w;
    } else {
        radius = p.y < 0.0 ? radii.y : radii.z;
    }
    float2 q = abs(p) - half_size + radius;
    return length(max(q, float2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

float4 blur_composite_fragment(BlurVertex input): SV_Target {
    float2 pt = input.position.xy;
    float2 clip_min = clip.xy;
    float2 clip_max = clip.xy + clip.zw;
    if (any(pt < clip_min) || any(pt >= clip_max)) {
        discard;
    }
    // Pixels whose centre is inside the shape; the quad drawn over this
    // anti-aliases the edge.
    if (rounded_rect_distance(pt) > 0.0) {
        discard;
    }
    float2 uv = (pt - region.xy) / region.zw;
    return blur_up(uv);
}
