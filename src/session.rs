use std::collections::HashSet;
use std::collections::HashMap;

use glutin::event::VirtualKeyCode;

use crate::level::*;
use crate::instance::*;
use crate::level_repository;
use crate::level_repository::LevelRepository;
use crate::level_repository::levels_path;
use crate::renderer::*;
use crate::manifest::*;
use crate::terminal::*;
use crate::lib::kinput::*;
use crate::lib::kmath::*;

// a high level thing for organizing gameplay. contains progress and level repository
pub struct Session {
    name: String,
    date: String,

    level_repository: LevelRepository,    
    completed_levels: HashSet<String>,

    current_level: Level,
    current_instance: Option<Instance>, // made off current level, if it exists we playing if not we editing
    // i want a terminal: open level, browse levels, save, load, etc

    tile_selection: Option<usize>,
    entity_selection: Option<usize>,

    terminal: Terminal,

    place_tokens: i32,
    place_link: String,
}

// lol present is such a macguffin, I wouldn't mind if there was something that made sense to happen once they were on a specific square, and you got a specific outcome and that was progress
// lol if you get a good job ribbon for finishing a level and you need to collect the praise
// santas workshop presents: good job, worldbuilding
// or good job / very good job for min moves


impl Session {
    pub fn new() -> Session {
        Session {
            name: "santa".to_owned(),
            date: "genesis".to_owned(),
            level_repository: LevelRepository::load(levels_path).unwrap_or(LevelRepository::new()),
            completed_levels: HashSet::new(),
            current_level: Level::from_string(noice_levels[0]).unwrap(),
            current_instance: None,

            tile_selection: None,
            entity_selection: None,

            terminal: Terminal::new(),

            place_tokens: 0,
            place_link: "void".to_owned(),
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) {
        rc.push(RenderCommand::solid_rect(inputs.screen_rect, Vec4::new(0.0, 0.0, 0.0, 1.0), 1.0));
        if let Some(ci) = &mut self.current_instance {
            // esc back to edit mode

            let outcome = ci.frame(inputs, rc, self.completed_levels.len() as i32, inputs.t as f32);
            if outcome != InstanceFrameOutcome::None {
                println!("outcome: {:?}", outcome);
            }
            match outcome {
                InstanceFrameOutcome::Completion(name) => {self.completed_levels.insert(name);},
                InstanceFrameOutcome::Bail => self.current_instance = None,
                InstanceFrameOutcome::Travel(dest) => self.current_instance = Some(Instance::new(self.level_repository.get_level(&dest).unwrap().instance())),
                InstanceFrameOutcome::None => {},
            }
        } else {
            let tiles = vec![Tile::Snow, Tile::Ice, Tile::Wall, Tile::Wall];
            let entities = vec![Entity::Player, Entity::Crate, Entity::Present, Entity::Receptacle, Entity::Tree, Entity::Portal(self.place_tokens, self.place_link.clone())];

            let pane_rect = inputs.screen_rect;

            let level_pane = pane_rect.fit_aspect_ratio(self.current_level.aspect().max(2.0)).fit_aspect_ratio(self.current_level.aspect());
            rc.push(RenderCommand::solid_rect(level_pane, Vec4::new(1.0, 0.0, 0.0, 1.0), 1.0));
            let level_rect = level_pane.dilate_pc(-0.04);
            self.current_level.render(level_rect, rc, self.completed_levels.len() as i32, 0.0);

            for i in 0..self.current_level.w {
                for j in 0..self.current_level.h {
                    let hover_rect = level_rect.grid_child(i, j, self.current_level.w, self.current_level.h);
                    if let Some(sel) = self.tile_selection {
                        if hover_rect.contains(inputs.mouse_pos) {
                            rc.push(RenderCommand {
                                colour: Vec4::new(1.0, 1.0, 1.0, 0.5),
                                pos: hover_rect,
                                sprite_clip: tile_clip(tiles[sel]),
                                depth: 4.0,
                            });
                        }
                        if hover_rect.contains(inputs.mouse_pos) && (inputs.lmb == KeyStatus::Pressed || inputs.lmb == KeyStatus::JustPressed) {
                            self.current_level.tiles[(j * self.current_level.w + i) as usize] = tiles[sel];
                        }
                    }
                    if let Some(sel) = self.entity_selection {
                        if hover_rect.contains(inputs.mouse_pos) {
                            rc.push(RenderCommand {
                                colour: Vec4::new(1.0, 1.0, 1.0, 0.5),
                                pos: hover_rect,
                                sprite_clip: entity_clip(&entities[sel]),
                                depth: 4.0,
                            });
                        }
                        if hover_rect.contains(inputs.mouse_pos) && (inputs.lmb == KeyStatus::Pressed || inputs.lmb == KeyStatus::JustPressed) {
                            if !self.current_level.entities.iter().any(|(e, ii, ij)| *e == entities[sel] && i == *ii && j == *ij) {
                                self.current_level.entities.push((entities[sel].clone(), i, j));
                            }
                        }
                    }
                    if hover_rect.contains(inputs.mouse_pos) && (inputs.rmb == KeyStatus::Pressed || inputs.rmb == KeyStatus::JustPressed) {
                        self.current_level.entities.retain(|(e, ii, ij)| i != *ii || j != *ij);
                    }
                }
            }

            let right_pane = Rect::new(level_pane.right(), 0.0, level_pane.x, inputs.screen_rect.h);

            {   
                if let Some(cmd) = self.terminal.frame(inputs, rc, right_pane) {
                    match cmd {
                        TerminalCommand::New(name) => {
                            if self.level_repository.contains_level(&name) {
                                self.terminal.tprint(format!("level {} already exists", name));
                            } else {
                                self.current_level = Level::new_empty(name);
                            }
                        },
                        TerminalCommand::Load(name) => {
                            if let Some(existing) = self.level_repository.get_level(&name) {
                                self.current_level = existing;
                            } else {
                                self.terminal.tprint(format!("level {} not found", name));
                            }
                        },
                        TerminalCommand::Save => {
                            self.level_repository.save_level(self.current_level.title.clone(), self.name.clone(), self.current_level.clone());
                        },
                        TerminalCommand::Play => {
                            self.level_repository.save_level(self.current_level.title.clone(), self.name.clone(), self.current_level.clone());
                            self.current_instance = Some(Instance::new(self.current_level.instance()));
                            // maybe check theres a player, an objective, etc
                        },
                        TerminalCommand::Reset => {
                            self.completed_levels = HashSet::new();
                        },
                        TerminalCommand::List => {
                            let mut level_names: Vec<String> = self.level_repository.data.keys().map(|x| x.clone()).collect();
                            level_names.sort();
                            for name in level_names {
                                let mut line = name.clone();
                                line.insert(0, ' ');
                                line.insert(0, ' ');
                                self.terminal.tprint(line);
                            }
                        },
                        TerminalCommand::Link(arg) => {
                            self.place_link = arg;
                        },
                        TerminalCommand::Tokens(arg) => {
                            self.place_tokens = arg as i32;
                        },
                        TerminalCommand::Dims(new_w, new_h) => {
                            let (old_w, old_h) = (self.current_level.w, self.current_level.h);
                            self.current_level.w = new_w as i32;
                            self.current_level.h = new_h as i32;
                            let old_tiles = self.current_level.tiles.clone();
                            self.current_level.tiles = vec![Tile::Wall; (new_w * new_h) as usize];
                            for i in 0..old_w {
                                for j in 0..old_h {
                                    if i < self.current_level.w as i32 && j < self.current_level.h as i32 {
                                        self.current_level.tiles[(j * self.current_level.w + i) as usize] = old_tiles[(j * old_w + i) as usize];
                                    }
                                }
                            }
                            // ensure entities dont go out of bounds
                            self.current_level.entities.retain(|(e, x, y)| *x < self.current_level.w && *y < self.current_level.h);
                        },
                    }
                }
                
            }



            {   // Left Pane
                let left_pane = Rect::new(0.0, 0.0, level_pane.x, pane_rect.h);
                let left_top = left_pane.child(0.0, 0.0, 1.0, 0.5);
                let tile_pane = left_top.fit_center_square();
                for i in 0..2 {
                    for j in 0..2 {
                        let tile_idx = (j * 2 + i) as usize;
                        // draw and if selected do it
                        let curr_tile_pane = tile_pane.grid_child(i, j, 2, 2);
                        let tile_rect = curr_tile_pane.dilate_pc(-0.04);
                        if tile_rect.contains(inputs.mouse_pos) && (inputs.lmb == KeyStatus::Pressed || inputs.lmb == KeyStatus::JustPressed) {
                            self.tile_selection = Some(tile_idx);
                            self.entity_selection = None;
                        }
                        if self.tile_selection == Some(tile_idx) {
                            rc.push(RenderCommand::solid_rect(curr_tile_pane, Vec4::new(1.0, 1.0, 0.0, 1.0), 1.5));
                        }
                        rc.push(RenderCommand {
                            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            pos: tile_rect,
                            sprite_clip: tile_clip(tiles[tile_idx]),
                            depth: 2.0,
                        });
                    }
                }

                let left_bot = left_pane.child(0.0, 0.5, 1.0, 0.5);
                let entity_pane = left_bot.fit_center_square();
                for i in 0..2 {
                    for j in 0..3 {
                        let entity_idx = (j * 2 + i) as usize;
                        let curr_entity_pane = entity_pane.grid_child(i, j, 2, 3);
                        let entity_rect = curr_entity_pane.dilate_pc(-0.04);
                        if entity_rect.contains(inputs.mouse_pos) && (inputs.lmb == KeyStatus::Pressed || inputs.lmb == KeyStatus::JustPressed) {
                            self.entity_selection = Some(entity_idx);
                            self.tile_selection = None;
                        }
                        if self.entity_selection == Some(entity_idx) {
                            rc.push(RenderCommand::solid_rect(curr_entity_pane, Vec4::new(1.0, 1.0, 0.0, 1.0), 1.5));
                        }
                        rc.push(RenderCommand {
                            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                            pos: entity_rect,
                            sprite_clip: entity_clip(&entities[entity_idx]),
                            depth: 2.0,
                        });
                    }
                }
            }
        }
    }
}