use bevy::{
    core_pipeline::core_3d::AlphaMask3d,
    pbr::{MeshPipelineKey, MeshUniform},
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::{PipelineCache, SpecializedMeshPipelines},
        view::{ExtractedView, VisibleEntities},
    },
};

use super::{draw, pipeline, voxel};

use crate::common::VoxelMesh;

#[allow(clippy::too_many_arguments)]
pub(crate) fn queue_voxel(
    alpha_mask_3d_draw_functions: Res<DrawFunctions<AlphaMask3d>>,
    voxel_pipeline: Res<pipeline::VoxelPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<pipeline::VoxelPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    voxel_mesh: Res<VoxelMesh>,
    voxels: Query<(Entity, &MeshUniform), With<voxel::Voxel>>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<AlphaMask3d>,
    )>,
) {
    let draw_custom = alpha_mask_3d_draw_functions
        .read()
        .get_id::<draw::DrawVoxels>()
        .unwrap();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);

    for (view, visible_entities, mut alpha_mask_phase) in views.iter_mut() {
        let rangefinder = view.rangefinder3d();

        if let Some(mesh) = meshes.get(&voxel_mesh.mesh) {
            let add_render_phase = |(entity, mesh_uniform): (Entity, &MeshUniform)| {
                let key =
                    msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines
                    .specialize(&mut pipeline_cache, &voxel_pipeline, key, &mesh.layout)
                    .unwrap();
                alpha_mask_phase.add(AlphaMask3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: rangefinder.distance(&mesh_uniform.transform),
                });
            };

            visible_entities
                .entities
                .iter()
                .filter_map(|visible_entity| voxels.get(*visible_entity).ok())
                .for_each(add_render_phase);
        }
    }
}
