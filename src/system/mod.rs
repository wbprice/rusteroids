mod collisions;
mod control_player;
mod lasers_damage_asteroids;
mod lasers_expire;
mod move_objects;
mod debug_boxes;

pub use self::{
    collisions::Collisions, control_player::ControlPlayer,
    lasers_damage_asteroids::LasersDamageAsteroids, lasers_expire::LasersExpire,
    move_objects::MoveObjects, debug_boxes::DebugBoxes
};
