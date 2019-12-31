use amethyst::ecs::{Component, DenseVecStorage};

pub struct Asteroid;

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}
