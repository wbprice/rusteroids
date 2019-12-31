use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    window::ScreenDimensions,
};

use crate::component::Velocity;

const OFFSCREEN_GRACE_ZONE: f32 = 32.0;

pub struct MoveObjects;

impl<'a> System<'a> for MoveObjects {
    type SystemData = (
        ReadExpect<'a, ScreenDimensions>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        Read<'a, Time>,
    );

    fn run(&mut self, (dimensions, mut transforms, velocities, time): Self::SystemData) {
        for (local, velocity) in (&mut transforms, &velocities).join() {
            // If the object has gone offscreen, teleport it to the other side of the screen
            let pos_x = local.translation().x;
            let pos_y = local.translation().y;

            // Offscreen to the left or right
            if pos_x < -OFFSCREEN_GRACE_ZONE {
                local.set_translation_x(dimensions.width() + OFFSCREEN_GRACE_ZONE);
            } else if pos_x > dimensions.width() + OFFSCREEN_GRACE_ZONE {
                local.set_translation_x(-OFFSCREEN_GRACE_ZONE);
            }

            // Offscreen to the top or bot
            if pos_y > dimensions.height() + OFFSCREEN_GRACE_ZONE {
                local.set_translation_y(-OFFSCREEN_GRACE_ZONE);
            } else if pos_y < -OFFSCREEN_GRACE_ZONE {
                local.set_translation_y(dimensions.height() + OFFSCREEN_GRACE_ZONE);
            }

            // Move the player ship according to the current velocity
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());
            local.prepend_rotation_z_axis(velocity.a * time.delta_seconds());
        }
    }
}
