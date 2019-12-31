use amethyst::{core::Transform, prelude::*, renderer::SpriteRender, window::ScreenDimensions};

use crate::component::{Asteroid, Velocity};

pub fn init_asteroid(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    // Center our sprites around the center of the window
    let x = dimensions.width() * 0.5;
    let y = dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // Create an entity for each sprite and attach the `SpriteRender` as
    // well as the transform. If you want to add behaviour to your sprites,
    // you'll want to add a custom `Component` that will identify them, and a
    // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html

    let asteroid_sprite = &sprites[1];

    world
        .create_entity()
        .with(asteroid_sprite.clone())
        .with(Asteroid {})
        .with(transform)
        .with(Velocity {
            x: -3.0,
            y: -2.0,
            a: -1.0,
        })
        .build();
}
