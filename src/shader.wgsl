struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct Camera {
  view_proj: mat2x2<f32>,
};

struct Transform {
    // @size(16) position: vec2<f32>, // pad to 16 bytes
    position: vec2<f32>,
    rotation: f32,
    scale: f32,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> transform: Transform;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let rotated_pos = vec2<f32>(
        model.position.x * cos(transform.rotation) - model.position.y * sin(transform.rotation),
        model.position.x * sin(transform.rotation) + model.position.y * cos(transform.rotation),
    );
    let scaled_pos = rotated_pos * transform.scale;
    let translated_pos = scaled_pos + transform.position;

    let pos = camera.view_proj * translated_pos;

    out.clip_position = vec4<f32>(pos, 1.0, 1.0);
    return out;
}

struct Material {
    color: vec4<f32>,
};

@group(1) @binding(1)
var<uniform> material: Material;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return material.color;
}
 
