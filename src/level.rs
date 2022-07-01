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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Entity {
    Player,
    Present,
    Crate,
    Receptacle,
    Portal(i32, String),
    Pow,
    Tree,
    TreeStump,
}

impl Entity {
    pub fn can_move(&self) -> bool {
        match self {
            Entity::Receptacle | Entity::Portal(_, _) | Entity::Tree | Entity::TreeStump => false,
            _ => true,
        }
    }
    pub fn player_allowed(&self) -> bool {
        match self {
            Entity::Receptacle | Entity::Portal(_, _) | Entity::Tree | Entity::TreeStump => true,
            _ => false,
        }
    }
    pub fn boxes_allowed(&self) -> bool {
        match self {
            Entity::Receptacle => true,
            _ => false,
        }
    }
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
    pub fn new_empty(title: String) -> Level {
        Level {
            title: title,
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
        entities.iter().any(|(e, i, j)| *e == Entity::Receptacle) &&
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

    pub fn render(&self, level_rect: Rect, rc: &mut Vec<RenderCommand>, num_tokens: i32) {
        render(level_rect, rc, self.w, self.h, &self.tiles, &self.entities, num_tokens)
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

    // this logic is kind of fucked lmao
    pub fn accept_move(&self, dir: (i32, i32), pos: (i32, i32)) -> bool {
        // let movers = self.current_entities
        // maybe I should have static entities as a separate thing
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        if self.l.tiles[(self.l.w * candidate_pos.1 + candidate_pos.0) as usize] == Tile::Wall {
            return false;
        }
        for e in self.current_entities.iter().filter_map(|(e, i, j)| if pos.0 == *i && pos.1 == *j { Some(e.clone()) } else { None }) {
            if e.can_move() {
                if e == Entity::Player && !self.current_entities.iter().any(|(e, i, j)| candidate_pos.0 == *i && candidate_pos.1 == *j && !e.player_allowed()) {
                    return true;
                } else if !self.current_entities.iter().any(|(e, i, j)| candidate_pos.0 == *i && candidate_pos.1 == *j && !e.boxes_allowed()) {
                    return true;
                } else {
                    return self.accept_move(dir, candidate_pos);        
                }
            }
        }
        return true;
    }

    pub fn apply_move(&mut self, dir: (i32, i32), pos: (i32, i32)) {
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        for i in 0..self.current_entities.len() {
            let (e, i, j) = &self.current_entities[i];
            // if we can move, first push the thing where we are moving
            if *i == candidate_pos.0 && *j == candidate_pos.1 && e.can_move() {
                // so this shouldnt even be happening when player moves up into nothing
                self.apply_move(dir, candidate_pos);        
                // if *e == Entity::Player && !self.current_entities.iter().any(|(e, i, j)| candidate_pos.0 == *i && candidate_pos.1 == *j && !e.player_allowed()) {
                // } else if !self.current_entities.iter().any(|(e, i, j)| candidate_pos.0 == *i && candidate_pos.1 == *j && !e.boxes_allowed()) {
                //     self.apply_move(dir, candidate_pos);        
                // }
            }
        }
        // do actual moving
        for e in self.current_entities.iter_mut() {
            if e.0.can_move() && e.1 == pos.0 && e.2 == pos.1 {
                *e = (e.0.clone(), candidate_pos.0, candidate_pos.1);
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
        let moving_players: Vec<usize> = self.current_entities.iter()
            .enumerate()
            .filter(|(idx, (e, i, j))| match e {Entity::Player => true, _ => false})
            .filter(|(idx, (e, i, j))| self.accept_move(dir, (*i, *j)))
            .map(|(idx, (e, i, j))| idx)
            .collect();
        
        if moving_players.len() == 0 {
            return false;
        }

        self.history.push(self.current_entities.clone());

        for mp in moving_players {
            self.apply_move(dir, (self.current_entities[mp].1, self.current_entities[mp].2));
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

        self.current_entities = self.history.pop().unwrap();

        return true;
    }

    pub fn victorious(&self) -> bool {
        Level::victorious(self.l.w, self.l.h, &self.l.tiles, &self.current_entities)
    }

    pub fn render(&self, level_rect: Rect, rc: &mut Vec<RenderCommand>, num_tokens: i32) {
        render(level_rect, rc, self.l.w, self.l.h, &self.l.tiles, &self.current_entities, num_tokens)
    }
}

fn render(level_rect: Rect, rc: &mut Vec<RenderCommand>, w: i32, h: i32, tiles: &[Tile], entities: &[(Entity, i32, i32)], num_tokens: i32) {
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
        let tile_rect = level_rect.grid_child(*i, *j, w, h);
        match e {
            Entity::Portal(tokens, link) => {
                rc.push(RenderCommand {
                    colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    sprite_clip: entity_clip(e),
                    pos: tile_rect,
                    depth: 2.5,
                });

                let portal_rect = tile_rect.child(3./16., 3./16., 10./16., 13./16.);
                if num_tokens < *tokens {
                    // inactive portal
                    let center = portal_rect.fit_aspect_ratio(2.);
                    let token_rect = center.grid_child(0, 0, 2, 1);
                    let num_rect = center.grid_child(1, 0, 2, 1);
                    rc.push(RenderCommand {
                        colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                        sprite_clip: Rect::new(3.0, 1.0, 1.0, 1.0),
                        pos: token_rect.dilate_pc(-0.1),
                        depth: 2.0,
                    });
                    render_text_center(format!("{}", tokens).as_bytes(), num_rect.dilate_pc(-0.1), 2.0, rc);
                } else {
                    // yea todo animate
                    let mut colour = Vec4::new(0.5, 0.0, 0.5, 1.0);
                    let final_colour = Vec4::new(0.8, 0.8, 0.5, 1.0);
                    let mut w = 0.7;
                    let mut h = 0.8;
                    let mut rect = portal_rect;
                    let mut depth = 1.2;
                    for i in 0..6 {
                        rc.push(RenderCommand::solid_rect(rect, colour, depth));
                        rect = rect.child(0.1, 0.1, w, h);
                        w *= 0.95;
                        h *= 0.95;
                        depth += 0.1;
                        colour = colour.lerp(final_colour, 0.1);
                    }
                }
            },
            Entity::Tree => {
                rc.push(RenderCommand {
                    colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    sprite_clip: entity_clip(e),
                    pos: tile_rect,
                    depth: 2.5,
                });
                rc.push(RenderCommand {
                    colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    sprite_clip: Rect::new(6.0, 2.0, 1.0, 1.0),
                    pos: tile_rect,
                    depth: 1.5,
                });
            },
            _ => rc.push(RenderCommand {
                colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                sprite_clip: entity_clip(e),
                pos: level_rect.grid_child(*i, *j, w, h),
                depth: match e {
                    Entity::Player | Entity::Present | Entity::Crate | Entity::Pow => 2.0,
                    Entity::Receptacle | Entity::Portal(_, _) => 1.5,
                    _ => 1.0,
                },
            }),
        }
    }
}

pub fn tile_clip(t: Tile) -> Rect {
    match t {
        Tile::Snow => Rect::new(1.0, 0.0, 1.0, 1.0),
        Tile::Ice => Rect::new(2.0, 0.0, 1.0, 1.0),
        Tile::Wall => Rect::new(0.0, 0.0, 1.0, 1.0),
    }
}

pub fn entity_clip(e: &Entity) -> Rect {
    match e {
        Entity::Player => Rect::new(4.0, 0.0, 1.0, 1.0),
        Entity::Present => Rect::new(3.0, 0.0, 1.0, 1.0),
        Entity::Receptacle => Rect::new(5.0, 0.0, 1.0, 1.0),
        Entity::Crate => Rect::new(6.0, 0.0, 1.0, 1.0),
        Entity::Portal(_,_) => Rect::new(7.0, 1.0, 1.0, 1.0),
        Entity::Pow => Rect::new(6.0, 1.0, 1.0, 1.0),
        Entity::Tree => Rect::new(7.0, 2.0, 1.0, 1.0),
        Entity::TreeStump => Rect::new(5.0, 2.0, 1.0, 1.0),
    }
}