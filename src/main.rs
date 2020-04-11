use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod component;
mod entity;
mod resource;
mod state;
mod system;

use crate::system::{
    Collisions, ControlPlayer, DebugBoxes, LasersDamageAsteroids, LasersDamageSmallAsteroids,
    LasersExpire, MoveObjects, ShipCollidesWithAsteroids
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");
    let binding_path = resources.join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(input_bundle)?
        .with(MoveObjects, "move_objects_system", &[])
        .with(ControlPlayer, "control_player_system", &[])
        .with(Collisions, "collisions_system", &[])
        .with(LasersExpire, "lasers_expire", &[])
        .with(LasersDamageAsteroids, "lasers_damage_asteroids", &[])
        .with(
            LasersDamageSmallAsteroids,
            "lasers_damage_small_asteroids",
            &[],
        )
        .with(ShipCollidesWithAsteroids, "ships_collide_with_asteroids", &[])
        .with(DebugBoxes, "debug_boxes", &[]);

    let mut game = Application::new(resources, state::MyState, game_data)?;
    game.run();

    Ok(())
}
