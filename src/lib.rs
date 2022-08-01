use bevy::{
    core_pipeline::Transparent3d,
    prelude::*,
    render::{
        render_asset::RenderAssetPlugin, render_component::ExtractComponentPlugin,
        render_phase::AddRenderCommand, render_resource::*, RenderApp, RenderStage,
    },
};

mod bundle;
mod draw;
mod extract_voxel_mesh_uniforms;
mod pipeline;
mod queue;
mod voxel;
mod voxel_mesh;
pub mod wireframe;

pub use bundle::VoxelBundle;
pub use voxel::{Voxel, VoxelData};

use extract_voxel_mesh_uniforms::extract_voxel_mesh_uniforms;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        let mut assets = app.world.resource_mut::<Assets<_>>();
        assets.set_untracked(
            pipeline::VOXEL_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("shaders/voxel.wgsl")),
        );

        app.add_plugin(ExtractComponentPlugin::<voxel::Voxel>::default())
            .add_asset::<VoxelData>()
            .add_plugin(RenderAssetPlugin::<VoxelData>::default())
            .init_resource::<voxel_mesh::VoxelMesh>();

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, draw::DrawVoxels>()
            .init_resource::<pipeline::VoxelPipeline>()
            .init_resource::<SpecializedMeshPipelines<pipeline::VoxelPipeline>>()
            .add_system_to_stage(RenderStage::Extract, voxel_mesh::extract_mesh)
            .add_system_to_stage(RenderStage::Extract, extract_voxel_mesh_uniforms)
            .add_system_to_stage(RenderStage::Queue, queue::queue_voxel);
    }
}
