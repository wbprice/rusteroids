use amethyst::ecs::{Component, DenseVecStorage};

pub struct Collidable {
    pub radius: f32
}

impl Component for Collidable {
    type Storage = DenseVecStorage<Self>;
}