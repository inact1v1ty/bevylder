use super::voxel;

use bevy::{
    pbr::{MeshUniform, NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::Extract,
};

bitflags::bitflags! {
    #[repr(transparent)]
    struct MeshFlags: u32 {
        const SHADOW_RECEIVER            = (1 << 0);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

pub fn extract_voxel_meshes(
    mut commands: Commands,
    mut prev_caster_commands_len: Local<usize>,
    mut prev_not_caster_commands_len: Local<usize>,
    meshes_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &GlobalTransform,
            With<voxel::Voxel>,
            Option<With<NotShadowReceiver>>,
            Option<With<NotShadowCaster>>,
        )>,
    >,
) {
    let mut caster_commands = Vec::with_capacity(*prev_caster_commands_len);
    let mut not_caster_commands = Vec::with_capacity(*prev_not_caster_commands_len);
    let visible_meshes = meshes_query.iter().filter(|(_, vis, ..)| vis.is_visible());

    for (entity, _, transform, _, not_receiver, not_caster) in visible_meshes {
        let transform = transform.compute_matrix();
        let shadow_receiver_flags = if not_receiver.is_some() {
            MeshFlags::empty().bits
        } else {
            MeshFlags::SHADOW_RECEIVER.bits
        };
        let uniform = MeshUniform {
            flags: shadow_receiver_flags,
            transform,
            inverse_transpose_model: transform.inverse().transpose(),
        };
        if not_caster.is_some() {
            not_caster_commands.push((entity, (uniform, NotShadowCaster)));
        } else {
            caster_commands.push((entity, (uniform,)));
        }
    }
    *prev_caster_commands_len = caster_commands.len();
    *prev_not_caster_commands_len = not_caster_commands.len();
    commands.insert_or_spawn_batch(caster_commands);
    commands.insert_or_spawn_batch(not_caster_commands);
}
