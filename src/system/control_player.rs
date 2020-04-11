use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::{
    component::{Collidable, Laser, Player, Velocity},
    resource::SpriteResource,
};

const THROTTLE_COEFFICIENT: f32 = 1.25;
const YAW_COEFFICIENT: f32 = 0.25;
const MAX_ANGULAR_VELOCITY: f32 = 3.0;

pub struct ControlPlayer;

impl<'a> System<'a> for ControlPlayer {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, SpriteResource>,
        WriteStorage<'a, SpriteRender>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Laser>,
        WriteStorage<'a, Collidable>,
    );

    fn run(
        &mut self,
        (
            input,
            sprite_resources,
            mut sprites,
            entities,
            players,
            mut transforms,
            mut velocities,
            mut lasers,
            mut collidables,
        ): Self::SystemData,
    ) {
        let throttle = input.axis_value("throttle");
        let steering = input.axis_value("steering");
        let lasers_firing = input.action_is_down("lasers");

        let mut ships_shooting: Vec<Transform> = vec![];

        for (_player, local, velocity) in (&players, &mut transforms, &mut velocities).join() {
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

            if let Some(lasers_firing) = lasers_firing {
                // Is the laser button down?
                if lasers_firing {
                    let lasers: Vec<&Laser> = (&lasers).join().collect();
                    if lasers.len() < 3 {
                        ships_shooting.push(local.clone());
                    }
                }
            }
        }

        for ship_local in ships_shooting {
            let transform = ship_local.clone();
            let (_, _, magnitude) = ship_local.rotation().euler_angles();

            let laser_displacement = 300.0;
            let velocity_x = magnitude.cos() * laser_displacement;
            let velocity_y = magnitude.sin() * laser_displacement;

            entities
                .build_entity()
                .with(
                    SpriteRender {
                        sprite_sheet: sprite_resources.sprite_sheet.clone(),
                        sprite_number: 2,
                    },
                    &mut sprites,
                )
                .with(transform, &mut transforms)
                .with(Laser::new(), &mut lasers)
                .with(Collidable { radius: 3.0 }, &mut collidables)
                .with(
                    Velocity {
                        x: velocity_x,
                        y: velocity_y,
                        a: 0.,
                    },
                    &mut velocities,
                )
                .build();
        }
    }
}
