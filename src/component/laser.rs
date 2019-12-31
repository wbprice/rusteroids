use amethyst::ecs::{Component, DenseVecStorage};

pub struct Laser;

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}
