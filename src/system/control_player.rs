use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::component::{Player, Velocity};

const THROTTLE_COEFFICIENT: f32 = 0.75;
const YAW_COEFFICIENT: f32 = 0.25;
const MAX_ANGULAR_VELOCITY: f32 = 3.0;

pub struct ControlPlayer;

impl<'a> System<'a> for ControlPlayer {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, players, transforms, mut velocities): Self::SystemData) {
        let throttle = input.axis_value("throttle");
        let steering = input.axis_value("steering");

        for (_player, local, velocity) in (&players, &transforms, &mut velocities).join() {
            if let Some(throttle) = throttle {
                // The new values for x and y velocity depend on the current heading
                // "magnitude" is [0; pi] where pi is a half rotation
                let (_, _, magnitude) = local.rotation().euler_angles();

                velocity.x += magnitude.cos() * throttle * THROTTLE_COEFFICIENT;
                velocity.y += magnitude.sin() * throttle * THROTTLE_COEFFICIENT;
            }

            if let Some(steering) = steering {
                // Steering input with scaling
                let angular_velocity = velocity.a + steering * YAW_COEFFICIENT;

                // Maximum angular velocity is 5.0
                if angular_velocity > 0.0 {
                    velocity.a = angular_velocity.min(MAX_ANGULAR_VELOCITY);
                } else {
                    velocity.a = angular_velocity.max(-MAX_ANGULAR_VELOCITY);
                }
            }
        }
    }
}
