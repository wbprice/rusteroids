use amethyst::{
    core::transform::Transform,
    ecs::{ReadStorage, System, WriteStorage},
};

use crate::component::{Asteroid, Velocity};

pub struct Collisions;

impl<'a> System<'a> for Collisions {
    type SystemData = (
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (_asteroids, mut _transforms, _velocities): Self::SystemData) {}
}
