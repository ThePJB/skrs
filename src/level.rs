use crate::lib::kmath::*;
use crate::renderer::*;
use serde::{Serialize, Deserialize};


// how am i going to do game logic and animate it?
// store prev and new and lerp
// allow player movement in next timesteps sure. it could be realtime lol.

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Snow,
    Ice,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Entity {
    Player,
    Present,
    Crate,
    Receptacle,
}

#[derive(Clone, Serialize, Deserialize)]

pub struct Level {
    pub title: String,
    pub w: i32,
    pub h: i32,
    pub tiles: Vec<Tile>,
    pub entities: Vec<(Entity, i32, i32)>,
}



impl Level {
    pub fn new_empty() -> Level {
        Level {
            title: "new level".to_owned(),
            w: 7,
            h: 7,
            tiles: vec![Tile::Wall; 49],
            entities: Vec::new(),
        }
    }
    pub fn from_string(s: &str) -> Option<Level> {
        let title = s.split("\n").nth(0)?.to_owned();
        let w = s.split("\n").nth(1)?.len();
        let h = s.split("\n").count() - 1;

        let mut it = s.split("\n"); it.next();
        for (line_number, line) in it.enumerate() {
            if line.len() != w {
                println!("bad line length: line {} length is {} (w is {})", line_number, line.len(), w);
                return None;
            }
        }
        if h == 0 {
            println!("no level content");
            return None;
        }

        let mut level = Level {
            w: w as i32,
            h: h as i32,
            tiles: vec![Tile::Snow; w*h],
            entities: vec![],
            title: title,
        };

        let mut it = s.split("\n"); it.next();
        for (j, line) in it.enumerate() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => level.tiles[j*w + i] = Tile::Wall,
                    ' ' => level.tiles[j*w + i] = Tile::Snow,
                    '/' => level.tiles[j*w + i] = Tile::Ice,
                    'p' => {
                        level.tiles[j*w + i] = Tile::Snow;
                        level.entities.push((Entity::Player, i as i32, j as i32));
                    },
                    'P' => {
                        level.tiles[j*w + i] = Tile::Ice;
                        level.entities.push((Entity::Player, i as i32, j as i32));
                    },
                    't' => {
                        level.tiles[j*w + i] = Tile::Snow;
                        level.entities.push((Entity::Receptacle, i as i32, j as i32));
                    },
                    'T' => {
                        level.tiles[j*w + i] = Tile::Ice;
                        level.entities.push((Entity::Receptacle, i as i32, j as i32));
                    },
                    'b' => {
                        level.tiles[j*w + i] = Tile::Snow;
                        level.entities.push((Entity::Present, i as i32, j as i32));
                    },
                    'B' => {
                        level.tiles[j*w + i] = Tile::Ice;
                        level.entities.push((Entity::Present, i as i32, j as i32));
                    },
                    'c' => {
                        level.tiles[j*w + i] = Tile::Snow;
                        level.entities.push((Entity::Crate, i as i32, j as i32));
                    },
                    'C' => {
                        level.tiles[j*w + i] = Tile::Ice;
                        level.entities.push((Entity::Crate, i as i32, j as i32));
                    },
                    _ => {
                        println!("forbidden chars in level!");
                        return None;
                    }
                }
            }
        }
        Some(level)
    }

    fn victorious(w: i32, h: i32, tiles: &[Tile], entities: &[(Entity, i32, i32)]) -> bool {
        entities.iter().all(|(e, i, j)| match e {
            Entity::Receptacle => {
                entities.iter().any(|(e, ii, ij)| {
                    match e {
                        Entity::Present => {
                            *i == *ii && *j == *ij
                        },
                        _ => false,
                    }
                })
            },
            _ => true,
        })
    }

    pub fn render(&self, level_rect: Rect, rc: &mut Vec<RenderCommand>) {
        render(level_rect, rc, self.w, self.h, &self.tiles, &self.entities)
    }

    pub fn aspect(&self) -> f32 {
        self.w as f32 / self.h as f32    
    }

    pub fn instance(&self) -> LevelInstance {
        LevelInstance { l: self.clone(), current_entities: self.entities.clone(), history: Vec::new(), momentum: Vec::new() }
    }
}

pub struct LevelInstance {
    pub l: Level,
    pub current_entities: Vec<(Entity, i32, i32)>,
    pub history: Vec<Vec<(Entity, i32, i32)>>,
    pub momentum: Vec<((i32, i32), (i32, i32))>,
}

impl LevelInstance {

    pub fn accept_move(&self, dir: (i32, i32), pos: (i32, i32)) -> bool {
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        if self.l.tiles[(self.l.w * candidate_pos.1 + candidate_pos.0) as usize] == Tile::Wall {
            return false;
        } else if self.l.entities.iter().any(|(e, i, j)| {
            candidate_pos.0 == *i && candidate_pos.1 == *j && match e {Entity::Receptacle => false, _ => true}
        }) {
            return self.accept_move(dir, candidate_pos)
        }
        return true;
    }

    pub fn apply_move(&mut self, dir: (i32, i32), pos: (i32, i32)) {
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        if self.l.entities.iter().any(|(e, i, j)| {
            candidate_pos.0 == *i && candidate_pos.1 == *j && match e {Entity::Receptacle => false, _ => true}
        }) {
            self.apply_move(dir, candidate_pos);
        }
        for e in self.l.entities.iter_mut() {
            if e.0 != Entity::Receptacle && e.1 == pos.0 && e.2 == pos.1 {
                *e = (e.0, candidate_pos.0, candidate_pos.1);
                if self.l.tiles[(candidate_pos.1 * self.l.w + candidate_pos.0) as usize] == Tile::Ice {
                    self.momentum.push((candidate_pos, dir));
                }
            }
        }
    }

    pub fn try_move(&mut self, dir: (i32, i32)) -> bool {
        self.momentum.clear();
        // return if move actually gets done
        // only make history if move actually gets done
        // move gets accepted if theres eventually an empty space
        let moving_players: Vec<usize> = self.l.entities.iter()
            .enumerate()
            .filter(|(idx, (e, i, j))| match e {Entity::Player => true, _ => false})
            .filter(|(idx, (e, i, j))| self.accept_move(dir, (*i, *j)))
            .map(|(idx, (e, i, j))| idx)
            .collect();
        
        if moving_players.len() == 0 {
            return false;
        }

        self.history.push(self.l.entities.clone());

        for mp in moving_players {
            self.apply_move(dir, (self.l.entities[mp].1, self.l.entities[mp].2));
        }

        while self.momentum.len() != 0 {
            let mm_copy = self.momentum.clone();
            self.momentum.clear();
            for (mpos, mdir) in mm_copy {
                if self.accept_move(mdir, mpos) {
                    self.apply_move(mdir, mpos);
                }
            }
        }

        return true;
    }

    pub fn undo(&mut self) -> bool {
        if self.history.len() == 0 {
            return false;
        }

        self.l.entities = self.history.pop().unwrap();

        return true;
    }

    pub fn victorious(&self) -> bool {
        Level::victorious(self.l.w, self.l.h, &self.l.tiles, &self.l.entities)
    }

    pub fn render(&self, level_rect: Rect, rc: &mut Vec<RenderCommand>) {
        render(level_rect, rc, self.l.w, self.l.h, &self.l.tiles, &self.l.entities)
    }
}

fn render(level_rect: Rect, rc: &mut Vec<RenderCommand>, w: i32, h: i32, tiles: &[Tile], entities: &[(Entity, i32, i32)]) {
    for i in 0..w {
        for j in 0..h {
            rc.push(RenderCommand {
                colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                sprite_clip: tile_clip(tiles[(j*w + i) as usize]), 
                pos: level_rect.grid_child(i, j, w, h),
                depth: 1.1,
            });
        }
    }
    for (e, i, j) in entities.iter() {
        rc.push(RenderCommand {
            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
            sprite_clip: entity_clip(*e),
            pos: level_rect.grid_child(*i, *j, w, h),
            depth: match e {
                Entity::Player | Entity::Present | Entity::Crate => 2.0,
                Entity::Receptacle => 1.5,
            },
        });
    }
}

pub fn tile_clip(t: Tile) -> Rect {
    match t {
        Tile::Snow => Rect::new(1.0, 0.0, 1.0, 1.0),
        Tile::Ice => Rect::new(2.0, 0.0, 1.0, 1.0),
        Tile::Wall => Rect::new(0.0, 0.0, 1.0, 1.0),
    }
}

pub fn entity_clip(e: Entity) -> Rect {
    match e {
        Entity::Player => Rect::new(4.0, 0.0, 1.0, 1.0),
        Entity::Present => Rect::new(3.0, 0.0, 1.0, 1.0),
        Entity::Receptacle => Rect::new(5.0, 0.0, 1.0, 1.0),
        Entity::Crate => Rect::new(6.0, 0.0, 1.0, 1.0),
    }
}