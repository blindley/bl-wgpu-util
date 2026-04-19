@group(0) @binding(0) var<uniform> uniform_color: vec4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let pos = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
    );

    let indices = array<u32, 6>(0, 1, 2, 2, 3, 0);

    return vec4<f32>(pos[indices[vertex_index]], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return uniform_color;
}