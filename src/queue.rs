use bevy::{
    core_pipeline::core_3d::Transparent3d,
    pbr::{MeshPipelineKey, MeshUniform},
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::{PipelineCache, SpecializedMeshPipelines},
        view::ExtractedView,
    },
};

use super::{draw, pipeline, voxel, voxel_mesh};

#[allow(clippy::too_many_arguments)]
pub(crate) fn queue_voxel(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    voxel_pipeline: Res<pipeline::VoxelPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<pipeline::VoxelPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    voxel_mesh: Res<voxel_mesh::VoxelMesh>,
    material_meshes: Query<(Entity, &MeshUniform), With<voxel::Voxel>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<draw::DrawVoxels>()
        .unwrap();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        if let Some(mesh) = meshes.get(&voxel_mesh.mesh) {
            for (entity, mesh_uniform) in material_meshes.iter() {
                let key =
                    msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines
                    .specialize(&mut pipeline_cache, &voxel_pipeline, key, &mesh.layout)
                    .unwrap();
                transparent_phase.add(Transparent3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                });
            }
        }
    }
}
