use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::component::{Player, Velocity};

pub struct ControlPlayer;

impl<'a> System<'a> for ControlPlayer {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, players, mut velocities): Self::SystemData) {
        let throttle = input.axis_value("throttle");
        let steering = input.axis_value("steering");

        // Should input handling be in it's own system?
        for (_player, velocity) in (&players, &mut velocities).join() {
            dbg!(&throttle);
            dbg!(&steering);

            if let Some(throttle) = throttle {
                velocity.y += throttle;
            }

            if let Some(steering) = steering {
                velocity.a += steering;
            }
        }
    }
}
