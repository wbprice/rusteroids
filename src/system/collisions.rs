use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{Asteroid, Velocity};

pub struct Collisions;

impl<'a> System<'a> for Collisions {
    type SystemData = (
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (asteroids, mut transforms, velocities): Self::SystemData) {}
}
