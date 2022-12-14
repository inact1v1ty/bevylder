// This code is heavily taken from bevy, which is licensed under either of
//
// * Apache License, Version 2.0, (../LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
// * MIT license (../LICENSE-MIT or http://opensource.org/licenses/MIT)
//
// at your option.

use bevy::asset::load_internal_asset;
use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::ecs::{prelude::*, reflect::ReflectComponent};
use bevy::pbr::MeshPipeline;
use bevy::pbr::{MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::prelude::*;
use bevy::reflect::{Reflect, TypeUuid};
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::view::VisibleEntities;
use bevy::render::Extract;
use bevy::render::{
    mesh::{Mesh, MeshVertexBufferLayout},
    render_asset::RenderAssets,
    render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
    render_resource::{
        PipelineCache, PolygonMode, RenderPipelineDescriptor, Shader, SpecializedMeshPipeline,
        SpecializedMeshPipelineError, SpecializedMeshPipelines,
    },
    view::{ExtractedView, Msaa},
    RenderApp, RenderStage,
};
use bevy::utils::tracing::error;

use super::{draw, voxel, voxel_mesh};

pub const WIREFRAME_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 6379866545205140404);

#[derive(Debug, Default)]
pub struct VoxelWireframePlugin;

impl Plugin for VoxelWireframePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            WIREFRAME_SHADER_HANDLE,
            "shaders/wireframe.wgsl",
            Shader::from_wgsl
        );

        app.init_resource::<VoxelWireframeConfig>()
            .init_resource::<VoxelWireframeConfig>()
            .add_plugin(ExtractResourcePlugin::<VoxelWireframeConfig>::default());

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_command::<Opaque3d, DrawWireframes>()
                .init_resource::<WireframePipeline>()
                .init_resource::<SpecializedMeshPipelines<WireframePipeline>>()
                .add_system_to_stage(RenderStage::Extract, extract_wireframes)
                .add_system_to_stage(RenderStage::Queue, queue_wireframes);
        }
    }
}

fn extract_wireframes(mut commands: Commands, query: Extract<Query<Entity, With<VoxelWireframe>>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(VoxelWireframe);
    }
}

/// Controls whether an entity should rendered in wireframe-mode if the [`VoxelWireframePlugin`] is enabled
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct VoxelWireframe;

#[derive(Debug, Clone, Default, ExtractResource, Reflect)]
#[reflect(Resource)]
pub struct VoxelWireframeConfig {
    /// Whether to show wireframes for all meshes. If `false`, only meshes with a [`VoxelWireframe`] component will be rendered.
    pub global: bool,
}

pub struct WireframePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}
impl FromWorld for WireframePipeline {
    fn from_world(render_world: &mut World) -> Self {
        WireframePipeline {
            mesh_pipeline: render_world.resource::<MeshPipeline>().clone(),
            shader: WIREFRAME_SHADER_HANDLE.typed(),
        }
    }
}

impl SpecializedMeshPipeline for WireframePipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;
        Ok(descriptor)
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn queue_wireframes(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    wireframe_config: Res<VoxelWireframeConfig>,
    wireframe_pipeline: Res<WireframePipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<WireframePipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    voxel_mesh: Res<voxel_mesh::VoxelMesh>,
    mut material_meshes: ParamSet<(
        Query<(Entity, &MeshUniform), With<voxel::Voxel>>,
        Query<(Entity, &MeshUniform), (With<voxel::Voxel>, With<VoxelWireframe>)>,
    )>,
    mut views: Query<(&ExtractedView, &VisibleEntities, &mut RenderPhase<Opaque3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<DrawWireframes>()
        .unwrap();
    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, visible_entities, mut opaque_phase) in views.iter_mut() {
        let rangefinder = view.rangefinder3d();

        let add_render_phase = |(entity, mesh_uniform): (Entity, &MeshUniform)| {
            if let Some(mesh) = render_meshes.get(&voxel_mesh.mesh) {
                let key =
                    msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline_id = pipelines.specialize(
                    &mut pipeline_cache,
                    &wireframe_pipeline,
                    key,
                    &mesh.layout,
                );
                let pipeline_id = match pipeline_id {
                    Ok(id) => id,
                    Err(err) => {
                        error!("{}", err);
                        return;
                    }
                };
                opaque_phase.add(Opaque3d {
                    entity,
                    pipeline: pipeline_id,
                    draw_function: draw_custom,
                    distance: rangefinder.distance(&mesh_uniform.transform),
                });
            }
        };

        if wireframe_config.global {
            let query = material_meshes.p0();
            visible_entities
                .entities
                .iter()
                .filter_map(|visible_entity| query.get(*visible_entity).ok())
                .for_each(add_render_phase);
        } else {
            let query = material_meshes.p1();
            visible_entities
                .entities
                .iter()
                .filter_map(|visible_entity| query.get(*visible_entity).ok())
                .for_each(add_render_phase);
        }
    }
}

type DrawWireframes = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    draw::DrawVoxel,
);
