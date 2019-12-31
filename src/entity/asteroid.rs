use amethyst::{core::Transform, prelude::*, renderer::SpriteRender, window::ScreenDimensions};
use rand::prelude::*;

use crate::component::{Asteroid, Velocity};

const MAX_ASTEROID_VELOCITY: f32 = 5.0;
const MAX_ASTEROID_ANGULAR_VELOCITY: f32 = 2.0;

pub fn init_asteroid(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    // It is known that the asteroid is the second sprite
    let asteroid_sprite = &sprites[1];

    // Place asteroids on the screen randomly.
    let mut rng = thread_rng();
    let x_coefficient: f32 = rng.gen();
    let y_coefficient: f32 = rng.gen();
    let x_velocity: f32 = rng.gen_range(-MAX_ASTEROID_VELOCITY, MAX_ASTEROID_VELOCITY);
    let y_velocity: f32 = rng.gen_range(-MAX_ASTEROID_VELOCITY, MAX_ASTEROID_VELOCITY);
    let a_velocity: f32 = rng.gen_range(
        -MAX_ASTEROID_ANGULAR_VELOCITY,
        MAX_ASTEROID_ANGULAR_VELOCITY,
    );

    // Center our sprites around the center of the window
    let x = dimensions.width() * x_coefficient;
    let y = dimensions.height() * y_coefficient;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    world
        .create_entity()
        .with(asteroid_sprite.clone())
        .with(Asteroid {})
        .with(transform)
        .with(Velocity {
            x: x_velocity,
            y: y_velocity,
            a: a_velocity,
        })
        .build();
}
