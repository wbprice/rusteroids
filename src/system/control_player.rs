use amethyst::{
    core::transform::{Parent, Transform},
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::{
    component::{Laser, Player, Velocity},
    resource::SpriteResource,
};

const THROTTLE_COEFFICIENT: f32 = 0.75;
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
        WriteStorage<'a, Parent>,
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
            mut parents,
        ): Self::SystemData,
    ) {
        let throttle = input.axis_value("throttle");
        let steering = input.axis_value("steering");
        let lasers_firing = input.action_is_down("lasers");

        let mut ships_shooting: Vec<Entity> = vec![];

        for (entity, player, local, velocity) in
            (&entities, &players, &mut transforms, &mut velocities).join()
        {
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
                    ships_shooting.push(entity)
                }
            }
        }

        for ship_entity in ships_shooting {
            let mut transform = Transform::default();
            transform.prepend_translation_x(24.0);

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
                .with(
                    Parent {
                        entity: ship_entity,
                    },
                    &mut parents,
                )
                .with(Laser {}, &mut lasers)
                .with(
                    Velocity {
                        x: 0.,
                        y: 0.,
                        a: 0.,
                    },
                    &mut velocities,
                )
                .build();
        }
    }
}
