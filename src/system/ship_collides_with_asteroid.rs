use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::component::{Asteroid, Player, SmallAsteroid};

const PLAYER_RADIUS: f32 = 24.0;
const ASTEROID_RADIUS: f32 = 36.0;
const SMALL_ASTEROID_RADIUS: f32 = 6.0;
pub struct ShipCollidesWithAsteroids;

impl<'a> System<'a> for ShipCollidesWithAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Asteroid>,
        ReadStorage<'a, SmallAsteroid>,
        ReadStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, players, asteroids, small_asteroids, transforms): Self::SystemData,
    ) {
        // Check for collisions with big asteroids
        for (_asteroid, asteroid_local) in (&asteroids, &transforms).join() {
            for (player_entity, _player, player_local) in (&entities, &players, &transforms).join()
            {
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;
                let player_x = player_local.translation().x;
                let player_y = player_local.translation().y;

                let dx = asteroid_x - player_x;
                let dy = asteroid_y - player_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < PLAYER_RADIUS + ASTEROID_RADIUS {
                    entities.delete(player_entity).unwrap();
                }
            }
        }

        // Check for collisions with small asteroids
        for (_asteroid, asteroid_local) in (&small_asteroids, &transforms).join() {
            for (player_entity, _player, player_local) in (&entities, &players, &transforms).join()
            {
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;
                let player_x = player_local.translation().x;
                let player_y = player_local.translation().y;

                let dx = asteroid_x - player_x;
                let dy = asteroid_y - player_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < PLAYER_RADIUS + SMALL_ASTEROID_RADIUS {
                    entities.delete(player_entity).unwrap();
                }
            }
        }
    }
}
