// how am i going to do game logic and animate it?
// store prev and new and lerp
// allow player movement in next timesteps sure. it could be realtime lol.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Snow,
    Ice,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Player,
    Present,
    Receptacle,
}

#[derive(Clone)]
pub struct Level {
    pub w: i32,
    pub h: i32,
    pub title: String,
    pub tiles: Vec<Tile>,
    pub entities: Vec<(Entity, i32, i32)>,
    pub history: Vec<Vec<(Entity, i32, i32)>>,
    pub momentum: Vec<((i32, i32), (i32, i32))>,
    // lerp off the history obviously
}

impl Level {
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
            history: vec![],
            momentum: vec![],
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
                    _ => {
                        println!("forbidden chars in level!");
                        return None;
                    }
                }
            }
        }
        Some(level)
    }

    pub fn accept_move(&self, dir: (i32, i32), pos: (i32, i32)) -> bool {
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        if self.tiles[(self.w * candidate_pos.1 + candidate_pos.0) as usize] == Tile::Wall {
            return false;
        } else if self.entities.iter().any(|(e, i, j)| {
            candidate_pos.0 == *i && candidate_pos.1 == *j && match e {Entity::Receptacle => false, _ => true}
        }) {
            return self.accept_move(dir, candidate_pos)
        }
        return true;
    }

    pub fn apply_move(&mut self, dir: (i32, i32), pos: (i32, i32)) {
        let candidate_pos = (dir.0 + pos.0, dir.1 + pos.1);
        if self.entities.iter().any(|(e, i, j)| {
            candidate_pos.0 == *i && candidate_pos.1 == *j && match e {Entity::Receptacle => false, _ => true}
        }) {
            self.apply_move(dir, candidate_pos);
        }
        for e in self.entities.iter_mut() {
            if e.0 != Entity::Receptacle && e.1 == pos.0 && e.2 == pos.1 {
                *e = (e.0, candidate_pos.0, candidate_pos.1);
                if self.tiles[(candidate_pos.1 * self.w + candidate_pos.0) as usize] == Tile::Ice {
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
        let moving_players: Vec<usize> = self.entities.iter()
            .enumerate()
            .filter(|(idx, (e, i, j))| match e {Entity::Player => true, _ => false})
            .filter(|(idx, (e, i, j))| self.accept_move(dir, (*i, *j)))
            .map(|(idx, (e, i, j))| idx)
            .collect();
        
        if moving_players.len() == 0 {
            return false;
        }

        self.history.push(self.entities.clone());

        for mp in moving_players {
            self.apply_move(dir, (self.entities[mp].1, self.entities[mp].2));
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

        self.entities = self.history.pop().unwrap();

        return true;
    }

    pub fn victorious(&self) -> bool {
        // all receptacles have a present at same coords
        self.entities.iter().all(|(e, i, j)| match e {
            Entity::Receptacle => {
                self.entities.iter().any(|(e, ii, ij)| {
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
}