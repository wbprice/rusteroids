use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::component::{Laser, SmallAsteroid};

pub struct LasersDamageSmallAsteroids;

const LASER_RADIUS: f32 = 3.0;
const SMALL_ASTEROID_RADIUS: f32 = 16.0;

impl<'a> System<'a> for LasersDamageSmallAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, SmallAsteroid>,
        ReadStorage<'a, Laser>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, small_asteroids, lasers, transforms): Self::SystemData) {
        for (small_asteroid_entity, _small_asteroid, small_asteroid_local) in
            (&entities, &small_asteroids, &transforms).join()
        {
            for (_laser, laser_entity, laser_local) in (&lasers, &entities, &transforms).join() {
                let laser_x = laser_local.translation().x;
                let laser_y = laser_local.translation().y;
                let small_asteroid_x = small_asteroid_local.translation().x;
                let small_asteroid_y = small_asteroid_local.translation().y;

                let dx = small_asteroid_x - laser_x;
                let dy = small_asteroid_y - laser_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < LASER_RADIUS + SMALL_ASTEROID_RADIUS {
                    entities.delete(small_asteroid_entity).unwrap();
                    entities.delete(laser_entity).unwrap();
                }
            }
        }
    }
}
