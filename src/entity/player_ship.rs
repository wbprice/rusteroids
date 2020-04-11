use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba, SpriteRender},
    window::ScreenDimensions,
};

use crate::component::{Collidable, Player, Velocity};

pub fn init_player_ship(
    world: &mut World,
    sprites: &[SpriteRender],
    dimensions: &ScreenDimensions,
) {
    // Center our sprites around the center of the window
    let x = 100. + dimensions.width() * 0.5;
    let y = 100. + dimensions.height() * 0.5;
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.);

    // Create an entity for each sprite and attach the `SpriteRender` as
    // well as the transform. If you want to add behaviour to your sprites,
    // you'll want to add a custom `Component` that will identify them, and a
    // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html

    let ship_sprite = &sprites[0];
    let mut debug_component = DebugLinesComponent::new();
    debug_component.add_circle_2d([x, y, 0.0].into(), 24.0, 12, Srgba::new(0.3, 0.3, 1.0, 1.0));

    world
        .create_entity()
        .with(ship_sprite.clone())
        .with(transform)
        .with(debug_component)
        .with(Player {})
        .with(Collidable { radius: 24.0 })
        .with(Velocity {
            x: 0.,
            y: 0.,
            a: 0.,
        })
        .build();
}
