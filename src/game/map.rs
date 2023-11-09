use crate::constant::{MAP_HEIGHT, MAP_WIGTH};

use super::element::Element;


// pub type Map = [[&'static str; MAP_WIGTH]; MAP_HEIGHT];
pub type Map = Vec<Vec<Box<dyn Element>>>;
