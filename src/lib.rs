use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_3d::AlphaMask3d,
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin, extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssetPlugin, render_phase::AddRenderCommand, render_resource::*,
        RenderApp, RenderStage,
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

use extract_voxel_mesh_uniforms::extract_voxel_meshes;
use pipeline::VOXEL_SHADER_HANDLE;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            VOXEL_SHADER_HANDLE,
            "shaders/voxel.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(ExtractComponentPlugin::<voxel::Voxel>::default())
            .add_asset::<VoxelData>()
            .add_plugin(RenderAssetPlugin::<VoxelData>::default())
            .add_plugin(ExtractResourcePlugin::<voxel_mesh::VoxelMesh>::default())
            .init_resource::<voxel_mesh::VoxelMesh>();

        app.sub_app_mut(RenderApp)
            .add_render_command::<AlphaMask3d, draw::DrawVoxels>()
            .init_resource::<pipeline::VoxelPipeline>()
            .init_resource::<SpecializedMeshPipelines<pipeline::VoxelPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_voxel_meshes)
            .add_system_to_stage(RenderStage::Queue, queue::queue_voxel);
    }
}
