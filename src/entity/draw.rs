use bevy::{
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    pbr::{SetMeshBindGroup, SetMeshViewBindGroup},
    prelude::*,
    render::{
        mesh::GpuBufferInfo,
        render_asset::RenderAssets,
        render_phase::{
            EntityRenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
        },
    },
};

use crate::common::{VoxelData, VoxelMesh};

use super::voxel;

pub(crate) type DrawVoxels = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetVoxelBindGroup<2>,
    DrawVoxel,
);

pub(crate) struct SetVoxelBindGroup<const I: usize>;

impl<const I: usize> EntityRenderCommand for SetVoxelBindGroup<I> {
    type Param = (SQuery<Read<voxel::Voxel>>, SRes<RenderAssets<VoxelData>>);

    fn render<'w>(
        _view: Entity,
        item: Entity,
        (voxels, voxel_data): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let voxel = voxels.get_inner(item).unwrap();
        let voxel_meta = voxel_data.into_inner().get(&voxel.data).unwrap();

        pass.set_bind_group(I, &voxel_meta.bind_group, &[]);

        RenderCommandResult::Success
    }
}

pub struct DrawVoxel;

impl EntityRenderCommand for DrawVoxel {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<VoxelMesh>);
    #[inline]
    fn render<'w>(
        _view: Entity,
        _item: Entity,
        (meshes, voxel_mesh): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let gpu_mesh = match meshes.into_inner().get(&voxel_mesh.mesh) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..1);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..1);
            }
        }
        RenderCommandResult::Success
    }
}
