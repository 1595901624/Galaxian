use crate::constant::{ENEMY_HEIGHT, ENEMY_WIGTH, MAP_HEIGHT, MAP_WIGTH, PLANE_POSITION};

use self::{
    element::{Element, EmptyElement},
    enemy::Enemy,
    map::Map,
    plane::Plane,
};

pub mod element;
pub mod enemy;
pub mod map;
pub mod plane;

/// Create Map
pub fn create_map() -> Map {
    let mut map: Map = vec![];
    let enemy_list = create_enemy_default();
    let plane = create_plane();
    for i in 0..MAP_HEIGHT {
        let mut temp: Vec<Box<dyn Element>> = vec![];
        for j in 0..MAP_WIGTH {
            temp.push(Box::new(EmptyElement::with(i, j)));
        }
        map.push(temp);
    }

    // insert enemy
    for ele in enemy_list {
        map[ele.position[0]][ele.position[1]] = Box::new(ele);
    }

    // insert plane
    map[plane.position[0]][plane.position[1]] = Box::new(plane);
    return map;
}

/// Create Plane
pub fn create_plane() -> Plane {
    return Plane::new(PLANE_POSITION, "▲");
}

// /// Create Enemy
// pub fn create_enemy() -> [[Enemy; ENEMY_WIGTH]; ENEMY_HEIGHT] {
//     return [];
// }

pub fn create_enemy_default() -> Vec<Enemy> {
    let mut list = vec![];
    for i in 2..4 {
        for j in 2..MAP_WIGTH - 3 {
            // 👻
            list.push(Enemy::new([i, j], "👻"));
        }
    }
    return list;
}

pub fn print_map(map: Map) {
    map.iter().for_each(|group| {
        group.iter().for_each(|item| {
            print!("{}", item.get_sign());
        });
        println!();
    });
}

// pub fn init_map_element() -> Map {

// }
