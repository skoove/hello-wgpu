var<private> VERTICES: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 2.0),
    vec2<f32>(1.73205, -1.0),
    vec2<f32>(-1.73205, -1.0),
);

struct Globals {
    resolution: vec2<f32>
};

@group(0) @binding(0)
var<uniform> globals: Globals;

// vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var out: VertexOutput;

    let pos = VERTICES[index];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);

    return out;
}

// frag shader

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / globals.resolution;

    let center = vec2<f32>(0.5, 0.5);
    let radius = 0.25; // 0.5 * 0.5
    let diff = uv - center;
    let dist_squared = dot(diff, diff);
    let edge_width = 0.01;

    var alpha: f32 = 0.0;
    if (dist_squared < radius) {
        alpha = 1.0;
    }

    return vec4<f32>(1.0, 1.0, 1.0, alpha);
}
