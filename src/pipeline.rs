use bevy::{
    pbr::{MeshPipeline, MeshPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
            BufferBindingType, BufferSize, RenderPipelineDescriptor, ShaderStages,
            SpecializedMeshPipeline, SpecializedMeshPipelineError,
        },
        renderer::RenderDevice,
    },
};

pub const VOXEL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7632171639263852275);

pub struct VoxelPipeline {
    pub(crate) shader: Handle<Shader>,
    pub(crate) mesh_pipeline: MeshPipeline,
    pub(crate) voxel_data_bind_group_layout: BindGroupLayout,
}

impl FromWorld for VoxelPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let render_device = world.get_resource_mut::<RenderDevice>().unwrap();
        let voxel_data_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("voxel data bind group"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            (std::mem::size_of::<u32>() as u64) * 16 * 16 * 16,
                        ),
                    },
                    count: None,
                }],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();
        VoxelPipeline {
            shader: VOXEL_SHADER_HANDLE.typed(),
            mesh_pipeline: mesh_pipeline.clone(),
            voxel_data_bind_group_layout,
        }
    }
}

impl SpecializedMeshPipeline for VoxelPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.voxel_data_bind_group_layout.clone(),
        ]);
        Ok(descriptor)
    }
}
