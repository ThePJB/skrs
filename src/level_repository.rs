use std::collections::HashMap;
use crate::level::*;
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::Read;
use std::io::Write;

// maybe overwrite protection or something would be smart

pub const levels_path: &str = "./levels.dat";

#[derive(Serialize, Deserialize)]
pub struct LevelRepository {
    data: HashMap<String, LevelMetadata>,
}

impl LevelRepository {
    pub fn load(path: &str) -> Option<LevelRepository> {
        print!("attempting to load levels.dat file...");
        let mut levels_file = File::open(path).ok()?;
        let mut contents = String::new();
        levels_file.read_to_string(&mut contents).ok();
        let repo = serde_json::from_str(&contents).ok()?;
        println!("ok");
        Some(repo)
    }
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

        // save levels to filesystem
        let str = serde_json::to_string(self).unwrap();
        let mut f = File::create(levels_path);
        if f.is_ok() {
            f.unwrap().write_all(str.as_bytes());
        }
    }
    pub fn print_levels(&self) {
        for k in self.data.keys(){
            println!("{}", k);
        }
    }
}

// needs load and save to file
#[derive(Serialize, Deserialize)]
pub struct LevelMetadata {
    creator: String,
    date: String,
    level: Level,
}