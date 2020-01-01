mod collisions;
mod control_player;
mod lasers_expire;
mod move_objects;

pub use self::{
    collisions::Collisions, control_player::ControlPlayer, lasers_expire::LasersExpire,
    move_objects::MoveObjects,
};
