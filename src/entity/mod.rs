use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_3d::AlphaMask3d,
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin, render_asset::RenderAssetPlugin,
        render_phase::AddRenderCommand, render_resource::*, RenderApp, RenderStage,
    },
};

pub mod bundle;
pub mod draw;
pub mod extract_voxel_mesh_uniforms;
pub mod pipeline;
pub mod queue;
pub mod voxel;

use extract_voxel_mesh_uniforms::extract_voxel_meshes;
use pipeline::VOXEL_SHADER_HANDLE;

use crate::common::VoxelData;

pub struct EntityVoxelPlugin;

impl Plugin for EntityVoxelPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            VOXEL_SHADER_HANDLE,
            "../shaders/voxel.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(ExtractComponentPlugin::<voxel::Voxel>::default())
            .add_asset::<VoxelData>()
            .add_plugin(RenderAssetPlugin::<VoxelData>::default());

        app.sub_app_mut(RenderApp)
            .add_render_command::<AlphaMask3d, draw::DrawVoxels>()
            .init_resource::<pipeline::VoxelPipeline>()
            .init_resource::<SpecializedMeshPipelines<pipeline::VoxelPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_voxel_meshes)
            .add_system_to_stage(RenderStage::Queue, queue::queue_voxel);
    }
}
