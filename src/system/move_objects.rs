use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Velocity};

pub struct MoveObjects;

impl<'a> System<'a> for MoveObjects {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        // Move the player ship according to the current velocity
        for (local, velocity) in (&mut transforms, &velocities).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());
            local.prepend_rotation_z_axis(velocity.a * time.delta_seconds());
        }
    }
}
