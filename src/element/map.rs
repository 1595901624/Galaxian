use crate::constant::{MAP_WIGTH, MAP_HEIGHT, PLANE_WIGTH, PLANE_HEIGHT};



pub type Map = [[&'static str; MAP_WIGTH]; MAP_HEIGHT];

pub type PlaneMap = [[&'static str; PLANE_WIGTH]; PLANE_HEIGHT];

// 游戏地图
pub const MAP: Map = [[""; MAP_WIGTH]; MAP_HEIGHT];
// 飞机地图
pub const PLANE_MAP: PlaneMap = [["X"; PLANE_WIGTH]; PLANE_HEIGHT];
