use bevy::prelude::*;

use super::voxel;

#[derive(Bundle, Clone)]
pub struct VoxelBundle {
    pub voxel: voxel::Voxel,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for VoxelBundle {
    fn default() -> Self {
        Self {
            voxel: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
