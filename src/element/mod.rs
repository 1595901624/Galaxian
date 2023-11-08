use crate::constant::{MAP_WIGTH, MAP_HEIGHT};

use self::map::Map;

pub(crate) mod map;
pub(crate) mod plane;

/// Create Map
pub(crate) fn create_map() -> Map {
    return [[""; MAP_WIGTH]; MAP_HEIGHT];
}

/// Create 
pub(crate) fn create_plane() {
    
}