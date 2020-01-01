use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::component::{Asteroid, Laser, Velocity};

pub struct LasersDamageAsteroids;

impl<'a> System<'a> for LasersDamageAsteroids {
    type SystemData = (
        WriteStorage<'a, Asteroid>,
        ReadStorage<'a, Laser>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (_asteroids, mut _transforms, _velocities): Self::SystemData) {}
}
