#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

[[group(1), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] vertex_position: vec3<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.vertex_position = vertex.position;
    return out;
}

struct Voxel {
    data: array<u32, 4096>;
};

[[group(2), binding(0)]]
var<uniform> voxel: Voxel;

fn to_color(byte: u32) -> vec4<f32> {
    let a = (byte >> 24u) & 0xFFu;
    let r = (byte >> 16u) & 0xFFu;
    let g = (byte >> 8u) & 0xFFu;
    let b = byte & 0xFFu;

    return vec4<f32>(f32(r) / 255.0, f32(g) / 255.0, f32(b) / 255.0, f32(a) / 255.0);
}

fn get(vpos: vec3<i32>) -> vec4<f32> {
    let uvpos = vec3<u32>(vpos);
    let idx = uvpos.x + uvpos.z * 16u + uvpos.y * 256u;
    return to_color(voxel.data[idx]);
}

fn intersect_plane_t(p: vec3<f32>, dir: vec3<f32>, plane: vec3<f32>) -> f32 {
    // FIXME
    let eps = 0.0000001;
    let d = dot(plane, dir);
    if (abs(d) > eps) {
        let w = p - plane;
        let fac = -dot(plane, w) / d;
        return fac;
    }
    return 1000.0;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let obj_space_camera_pos = (vec4<f32>(view.world_position, 1.0) * mesh.inverse_transpose_model).xyz;
    let view_dir = normalize(in.vertex_position - obj_space_camera_pos);
    var vpos = vec3<i32>((in.vertex_position + 0.5) * 15.9999);
    var color = get(vpos);

    if (color.a > 0.5) {
        return vec4<f32>(color.rgb, 0.0);
    }

    let vsign = sign(view_dir);
    let step = vec3<i32>(vsign);
    let t_delta = 1.0 / abs(view_dir);

    let sub_pos = 2.0 * ((in.vertex_position + 0.5) * 16.0 - vec3<f32>(vpos)) - 1.0;
    var t_max = 0.5 * vec3<f32>(
        intersect_plane_t(sub_pos, view_dir, vec3<f32>(vsign.x, 0.0, 0.0)),
        intersect_plane_t(sub_pos, view_dir, vec3<f32>(0.0, vsign.y, 0.0)),
        intersect_plane_t(sub_pos, view_dir, vec3<f32>(0.0, 0.0, vsign.z)),
    );

    loop {
        if (t_max.x < t_max.y) {
            if (t_max.x < t_max.z) {
                vpos.x = vpos.x + step.x;
                if (vpos.x < 0 || vpos.x >= 16) { return vec4<f32>(0.0, 0.0, 0.0, 0.0); }
                t_max.x = t_max.x + t_delta.x;
            } else {
                vpos.z = vpos.z + step.z;
                if (vpos.z < 0 || vpos.z >= 16) { return vec4<f32>(0.0, 0.0, 0.0, 0.0); }
                t_max.z = t_max.z + t_delta.z;
            }
        } else {
            if (t_max.y < t_max.z) {
                vpos.y = vpos.y + step.y;
                if (vpos.y < 0 || vpos.y >= 16) { return vec4<f32>(0.0, 0.0, 0.0, 0.0); }
                t_max.y = t_max.y + t_delta.y;
            } else {
                vpos.z = vpos.z + step.z;
                if (vpos.z < 0 || vpos.z >= 16) { return vec4<f32>(0.0, 0.0, 0.0, 0.0); }
                t_max.z = t_max.z + t_delta.z;
            }
        }
        color = get(vpos);

        if (color.a > 0.5) {
            return vec4<f32>(color.rgb, 0.0);
        }
    }
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
