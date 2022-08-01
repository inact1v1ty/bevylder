use bevy::prelude::*;

#[derive(Clone)]
pub struct VoxelMesh {
    pub(crate) mesh: Handle<Mesh>,
}

impl FromWorld for VoxelMesh {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

        VoxelMesh { mesh }
    }
}

pub fn extract_mesh(mut commands: Commands, voxel_mesh: Res<VoxelMesh>) {
    commands.insert_resource(voxel_mesh.clone());
}
