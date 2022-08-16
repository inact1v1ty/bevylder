use bevy::reflect::TypeUuid;

mod voxel_mesh;

pub use voxel_mesh::VoxelMesh;

#[repr(transparent)]
#[derive(Clone, TypeUuid)]
#[uuid = "180e10f3-5c78-43ed-8b46-69af97071fdc"]
pub struct VoxelData(pub [u32; 16 * 16 * 16]);
