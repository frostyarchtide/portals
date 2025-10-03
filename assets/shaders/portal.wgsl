#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let viewport_uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);
    
    if (mesh.uv.x < 0.05 || mesh.uv.x > 0.95 || mesh.uv.y < 0.025 || mesh.uv.y > 0.975) {
        return vec4(1.0, 0.5, 0.0, 1.0);
    }

    return textureSample(material_color_texture, material_color_sampler, viewport_uv);
}

