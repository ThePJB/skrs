use std::collections::HashMap;
use crate::level::*;

pub struct LevelRepository {
    data: HashMap<String, LevelMetadata>,
}

impl LevelRepository {
    pub fn new() -> LevelRepository {
        LevelRepository {
            data: HashMap::new(),
        }
    }
    pub fn contains_level(&self, name: String) -> bool{
        self.data.contains_key(&name)
    }
    pub fn get_level(&self, name: String) -> Option<Level> {
        self.data.get(&name).map(|x| x.level.clone())
    }
    pub fn save_level(&mut self, name: String, creator: String, level: Level) {
        let date: String = chrono::offset::Local::now().to_string();
        let md = LevelMetadata {
            creator,
            date,
            level,
        };
        self.data.insert(name, md);
    }
}

// needs load and save to file

pub struct LevelMetadata {
    creator: String,
    date: String,
    level: Level,
}