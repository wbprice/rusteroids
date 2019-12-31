use amethyst::{
    core::{Transform, Parent},
    ecs::prelude::{Entity},
    prelude::*,
    renderer::SpriteRender,
    window::ScreenDimensions,
};

use crate::component::{Laser, Velocity};

pub fn init_laser(
    world: &mut World,
    sprites: &[SpriteRender],
    parent: Entity,
    dimensions: &ScreenDimensions,
) {
    // Center our sprites around the center of the window
    let x = 100. + dimensions.width() * 0.5;
    let y = 100. + dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    let laser_sprite = &sprites[2];

    world
        .create_entity()
        .with(laser_sprite.clone())
        .with(transform)
        .with(Parent { entity: parent })
        .with(Laser {})
        .with(Velocity {
            x: 0.,
            y: 0.,
            a: 0.,
        })
        .build();
}
