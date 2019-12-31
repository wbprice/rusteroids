use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::component::{Player, Velocity};

pub struct MovePlayer;

impl<'a> System<'a> for MovePlayer {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        Read<'a, Time>
    );

    fn run(&mut self, (players, mut transforms, velocities, time): Self::SystemData) {
        // Move the player ship according to the current velocity
        for (_player, local, velocity) in (&players, &mut transforms, &velocities).join() {
            local.prepend_translation_x(velocity.x * time.delta_seconds());
            local.prepend_translation_y(velocity.y * time.delta_seconds());
        }
    }
}
