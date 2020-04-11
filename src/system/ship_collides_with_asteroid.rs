use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::component::{Asteroid, Player, SmallAsteroid, Collidable};

pub struct ShipCollidesWithAsteroids;

impl<'a> System<'a> for ShipCollidesWithAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Asteroid>,
        ReadStorage<'a, SmallAsteroid>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collidable>
    );

    fn run(
        &mut self,
        (entities, players, asteroids, small_asteroids, transforms, collidables): Self::SystemData,
    ) {
        // Check for collisions with big asteroids
        for (_asteroid, asteroid_local, asteroid_collidable) in (&asteroids, &transforms, &collidables).join() {
            for (player_entity, _player, player_local, player_collidable) in (&entities, &players, &transforms, &collidables).join()
            {
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;
                let player_x = player_local.translation().x;
                let player_y = player_local.translation().y;

                let dx = asteroid_x - player_x;
                let dy = asteroid_y - player_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < player_collidable.radius + asteroid_collidable.radius {
                    entities.delete(player_entity).unwrap();
                }
            }
        }

        // Check for collisions with small asteroids
        for (_asteroid, asteroid_local, asteroid_collidable) in (&small_asteroids, &transforms, &collidables).join() {
            for (player_entity, _player, player_local, player_collidable) in (&entities, &players, &transforms, &collidables).join()
            {
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;
                let player_x = player_local.translation().x;
                let player_y = player_local.translation().y;

                let dx = asteroid_x - player_x;
                let dy = asteroid_y - player_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < player_collidable.radius + asteroid_collidable.radius {
                    entities.delete(player_entity).unwrap();
                }
            }
        }
    }
}
