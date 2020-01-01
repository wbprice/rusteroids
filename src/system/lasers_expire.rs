use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
};

use crate::component::Laser;

pub struct LasersExpire;

impl<'a> System<'a> for LasersExpire {
    type SystemData = (Entities<'a>, WriteStorage<'a, Laser>, Read<'a, Time>);

    fn run(&mut self, (entities, mut lasers, time): Self::SystemData) {
        for (entity, laser) in (&entities, &mut lasers).join() {
            laser.ttl -= time.delta_seconds();

            if laser.ttl < 0.0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}
