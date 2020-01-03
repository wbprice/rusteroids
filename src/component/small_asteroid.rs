use amethyst::ecs::{Component, DenseVecStorage};

pub struct SmallAsteroid;

impl Component for SmallAsteroid {
    type Storage = DenseVecStorage<Self>;
}
