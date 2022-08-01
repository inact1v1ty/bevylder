use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_component::ExtractComponent,
        render_resource::{
            internal::bytemuck, BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer,
            BufferInitDescriptor, BufferUsages,
        },
        renderer::RenderDevice,
    },
};

use super::pipeline;

#[repr(transparent)]
#[derive(Clone, TypeUuid)]
#[uuid = "180e10f3-5c78-43ed-8b46-69af97071fdc"]
pub struct VoxelData(pub [u32; 16 * 16 * 16]);

#[derive(Clone)]
pub struct VoxelMeta {
    pub(crate) _buffer: Buffer,
    pub(crate) bind_group: BindGroup,
}

impl Default for VoxelData {
    fn default() -> Self {
        Self([0u32; 16 * 16 * 16])
    }
}

impl RenderAsset for VoxelData {
    type ExtractedAsset = Self;
    type PreparedAsset = VoxelMeta;

    type Param = (SRes<RenderDevice>, SRes<pipeline::VoxelPipeline>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("voxel data buffer"),
            contents: bytemuck::cast_slice(&extracted_asset.0),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("voxel data bind group"),
            layout: &pipeline.voxel_data_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Ok(VoxelMeta {
            _buffer: buffer,
            bind_group,
        })
    }
}

#[derive(Component, Clone, Default)]
pub struct Voxel {
    pub data: Handle<VoxelData>,
}

impl ExtractComponent for Voxel {
    type Query = &'static Self;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        item.clone()
    }
}
