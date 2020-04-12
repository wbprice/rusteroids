use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Entities, Join, Read, ReadExpect, System, Write, WriteStorage},
    renderer::SpriteRender,
    window::ScreenDimensions,
};

use crate::{
    component::{Collidable, Player, Velocity},
    resource::SpriteResource,
    state::RespawnTimer,
};

pub struct ShipRespawns;

impl<'a> System<'a> for ShipRespawns {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, ScreenDimensions>,
        ReadExpect<'a, SpriteResource>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Collidable>,
        WriteStorage<'a, Velocity>,
        Write<'a, RespawnTimer>,
        Read<'a, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            dimensions,
            sprite_resources,
            mut sprites,
            mut transforms,
            mut players,
            mut collidables,
            mut velocities,
            mut respawn_timer,
            time,
        ): Self::SystemData,
    ) {
        let player_count = (&entities, &players).join().count();
        if player_count == 0 {
            if respawn_timer.time_remaining > 0.0 {
                respawn_timer.time_remaining = respawn_timer.time_remaining - time.delta_seconds();
            } else {
                // Spawn another player
                let x = 100. + dimensions.width() * 0.5;
                let y = 100. + dimensions.height() * 0.5;
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.);

                entities
                    .build_entity()
                    .with(
                        SpriteRender {
                            sprite_sheet: sprite_resources.sprite_sheet.clone(),
                            sprite_number: 0,
                        },
                        &mut sprites,
                    )
                    .with(transform, &mut transforms)
                    .with(Player {}, &mut players)
                    .with(Collidable { radius: 24.0 }, &mut collidables)
                    .with(
                        Velocity {
                            x: 0.,
                            y: 0.,
                            a: 0.,
                        },
                        &mut velocities,
                    )
                    .build();

                // Reset the timer
                respawn_timer.time_remaining = 3.0;
            }
        }
    }
}
