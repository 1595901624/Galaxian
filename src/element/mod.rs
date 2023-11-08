use crate::constant::{MAP_HEIGHT, MAP_WIGTH, PLANE_POSITION, ENEMY_WIGTH, ENEMY_HEIGHT};

use self::{map::Map, plane::Plane, enemy::Enemy};

pub(crate) mod map;
pub(crate) mod plane;
pub(crate) mod enemy;

/// Create Map
pub(crate) fn create_map() -> Map {
    return [[""; MAP_WIGTH]; MAP_HEIGHT];
}

/// Create Plane
pub(crate) fn create_plane() -> Plane {
    return Plane::new(PLANE_POSITION);
}

// /// Create Enemy
// pub(crate) fn create_enemy() -> [[Enemy; ENEMY_WIGTH]; ENEMY_HEIGHT] {
//     return [];
// }

// pub(crate) fn create_enemy_default() -> [[Enemy; ENEMY_WIGTH]; ENEMY_HEIGHT] {
//     return [];
// }
