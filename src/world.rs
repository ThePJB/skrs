use crate::{level::*, manifest::{noice_levels, ice_levels}};

pub struct World {
    pub title: String,
    pub levels: Vec<Level>,
    pub completion: Vec<bool>,
}

impl World {
    pub fn world1() -> World {
        World {
            title: "world1".to_owned(),
            levels: vec![
                Level::from_string(noice_levels[0]).unwrap(),
                Level::from_string(noice_levels[1]).unwrap(),
                Level::from_string(noice_levels[2]).unwrap(),
                Level::from_string(noice_levels[3]).unwrap(),
                Level::from_string(noice_levels[4]).unwrap(),
                Level::from_string(noice_levels[5]).unwrap(),
            ], 
            completion: vec![false; 6] }
    }
    pub fn world2() -> World {
        World {
            title: "world2".to_owned(),
            levels: vec![
                Level::from_string(ice_levels[0]).unwrap(),
                Level::from_string(ice_levels[1]).unwrap(),
                Level::from_string(ice_levels[2]).unwrap(),
                Level::from_string(ice_levels[3]).unwrap(),
                Level::from_string(ice_levels[4]).unwrap(),
                Level::from_string(ice_levels[5]).unwrap(),
                Level::from_string(ice_levels[6]).unwrap(),
                Level::from_string(ice_levels[7]).unwrap(),
            ], 
            completion: vec![false; 8] }
    }
}