use amethyst::{
    core::transform::Transform,
    ecs::{ReadStorage, System, WriteStorage},
};

use crate::component::{Player, Velocity};

pub struct MovePlayer;

impl<'a> System<'a> for MovePlayer {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (players, transforms, velocities): Self::SystemData) {}
}
