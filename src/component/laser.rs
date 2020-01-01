use amethyst::ecs::{Component, DenseVecStorage};

pub struct Laser {
    pub ttl: f32,
}

impl Laser {
    pub fn new() -> Laser {
        Laser { ttl: 1.0 }
    }
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}
