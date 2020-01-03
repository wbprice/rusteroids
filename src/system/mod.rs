mod collisions;
mod control_player;
mod debug_boxes;
mod lasers_damage_asteroids;
mod lasers_expire;
mod move_objects;

pub use self::{
    collisions::Collisions, control_player::ControlPlayer, debug_boxes::DebugBoxes,
    lasers_damage_asteroids::LasersDamageAsteroids, lasers_expire::LasersExpire,
    move_objects::MoveObjects,
};
