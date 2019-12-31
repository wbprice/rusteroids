use amethyst::{
    ecs::{prelude::*, System},
    prelude::*,
};

pub struct MovePlayer;

impl<'a> System<'a> for MovePlayer {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        println!("Hello!");
    }
}
