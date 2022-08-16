use bevy::{prelude::*, render::extract_resource::ExtractResource};

#[derive(ExtractResource, Clone)]
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
