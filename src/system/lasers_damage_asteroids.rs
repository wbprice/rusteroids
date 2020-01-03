use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::component::{Asteroid, Laser, Velocity};

pub struct LasersDamageAsteroids;

const LASER_RADIUS: f32 = 3.0;
const ASTEROID_RADIUS: f32 = 36.0;

impl<'a> System<'a> for LasersDamageAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Asteroid>,
        ReadStorage<'a, Laser>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, asteroids, lasers, transforms): Self::SystemData) {
        for (asteroid_entity, _asteroid, asteroid_local) in
            (&entities, &asteroids, &transforms).join()
        {
            for (_laser, laser_local) in (&lasers, &transforms).join() {
                let laser_x = laser_local.translation().x;
                let laser_y = laser_local.translation().y;
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;

                let dx = asteroid_x - laser_x;
                let dy = asteroid_y - laser_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < LASER_RADIUS + ASTEROID_RADIUS {
                    dbg!("collision!");
                    entities.delete(asteroid_entity).unwrap();
                    // TODO: spawn two small asteroids
                }
            }
        }
    }
}
