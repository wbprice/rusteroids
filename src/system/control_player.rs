use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::component::{Player, Velocity};

pub struct ControlPlayer;

impl<'a> System<'a> for ControlPlayer {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, players, mut velocities): Self::SystemData) {
        let throttle = input.axis_value("throttle");
        let steering = input.axis_value("steering");

        // Should input handling be in it's own system?
        for (_player, velocity) in (&players, &mut velocities).join() {
            if let Some(throttle) = throttle {
                velocity.y += throttle;
            }

            if let Some(steering) = steering {
                // Steering input with scaling
                let angular_velocity = velocity.a + steering * 0.5;

                // Maximum angular velocity is 5.0
                if angular_velocity > 0.0 {
                    velocity.a = angular_velocity.min(5.0);
                } else {
                    velocity.a = angular_velocity.max(-5.0);
                }
            }
        }
    }
}
