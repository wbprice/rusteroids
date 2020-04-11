mod collisions;
mod control_player;
mod debug_boxes;
mod lasers_damage_asteroids;
mod lasers_damage_small_asteroids;
mod lasers_expire;
mod move_objects;
mod ship_collides_with_asteroid;

pub use self::{
    collisions::Collisions, control_player::ControlPlayer, debug_boxes::DebugBoxes,
    lasers_damage_asteroids::LasersDamageAsteroids,
    lasers_damage_small_asteroids::LasersDamageSmallAsteroids, lasers_expire::LasersExpire,
    move_objects::MoveObjects, ship_collides_with_asteroid::ShipCollidesWithAsteroids,
};
