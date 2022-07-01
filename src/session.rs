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

    terminal_history: Vec<String>,
    terminal_str: String,
    history_idx: Option<i32>,

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

            terminal_history: Vec::new(),
            terminal_str: "".to_owned(),
            history_idx: None,

            place_tokens: 0,
            place_link: "void".to_owned(),
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) {
        if let Some(ci) = &mut self.current_instance {
            // esc back to edit mode

            let outcome = ci.frame(inputs, rc, self.completed_levels.len() as i32);
            println!("outcome: {:?}", outcome);
            match outcome {
                InstanceFrameOutcome::Completion(name) => {self.completed_levels.insert(name);},
                InstanceFrameOutcome::Bail => self.current_instance = None,
                InstanceFrameOutcome::Travel(dest) => self.current_instance = Some(Instance::new(self.level_repository.get_level(dest).unwrap().instance())),
                InstanceFrameOutcome::None => {},
            }
        } else {
            let tiles = vec![Tile::Snow, Tile::Ice, Tile::Wall, Tile::Wall];
            let entities = vec![Entity::Player, Entity::Crate, Entity::Present, Entity::Receptacle, Entity::Tree, Entity::Portal(self.place_tokens, self.place_link.clone())];

            let pane_rect = inputs.screen_rect.child(0., 0.00, 1.0, 0.92);
            let term_rect = inputs.screen_rect.child(0., 0.92, 1.0, 0.08);

            let level_pane = pane_rect.fit_aspect_ratio(self.current_level.aspect().max(2.0)).fit_aspect_ratio(self.current_level.aspect());
            rc.push(RenderCommand::solid_rect(level_pane, Vec4::new(1.0, 0.0, 0.0, 1.0), 1.0));
            let level_rect = level_pane.dilate_pc(-0.04);
            self.current_level.render(level_rect, rc, self.completed_levels.len() as i32);

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

            {   // terminal
                // wants an edit method so that on edit we reset the history idx
                let edit = |c, s: &mut String, h: &mut Option<i32>| {
                    s.push(c);
                    *h = None;
                };
                rc.push(RenderCommand::solid_rect(term_rect, Vec4::new(0.2, 0.2, 0.2, 1.0), 2.0));
                for keystroke in inputs.keys_pressed_this_frame.iter() {
                    match keystroke {
                        VirtualKeyCode::Q => edit('q', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::W => edit('w', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::E => edit('e', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::R => edit('r', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::T => edit('t', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Y => edit('y', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::U => edit('u', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::I => edit('i', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::O => edit('o', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::P => edit('p', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::A => edit('a', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::S => edit('s', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::D => edit('d', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::F => edit('f', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::G => edit('g', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::H => edit('h', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::J => edit('j', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::K => edit('k', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::L => edit('l', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Z => edit('z', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::X => edit('x', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::C => edit('c', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::V => edit('v', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::B => edit('b', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::N => edit('n', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::M => edit('m', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Space => edit(' ', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key1 => edit('1', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key2 => edit('2', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key3 => edit('3', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key4 => edit('4', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key5 => edit('5', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key6 => edit('6', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key7 => edit('7', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key8 => edit('8', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key9 => edit('9', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Key0 => edit('0', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Minus => edit('-', &mut self.terminal_str, &mut self.history_idx),
                        VirtualKeyCode::Tab => {},// try autocomplete
                        VirtualKeyCode::Return => {
                            // feedback is gonna be important esp wrt overwriting
                            // save vs saveas vs overwrite
                            // play should autosave
                            if !self.terminal_str.is_empty() {
                                if self.terminal_str.starts_with("open ") && self.terminal_str.split(" ").count() == 2 {
                                    let level_name = self.terminal_str.split(" ").nth(1).unwrap().to_owned();
                                    self.current_level = self.level_repository.get_level(level_name.clone()).unwrap_or(Level::new_empty(level_name));
                                }
                                if self.terminal_str.starts_with("link ") && self.terminal_str.split(" ").count() == 2 {
                                    let arg = self.terminal_str.split(" ").nth(1).unwrap().to_owned();
                                    self.place_link = arg;
                                }
                                if self.terminal_str.starts_with("tokens ") && self.terminal_str.split(" ").count() == 2 {
                                    let arg = self.terminal_str.split(" ").nth(1).unwrap().to_owned();
                                    if let Ok(num) = arg.parse::<u32>() {
                                        self.place_tokens = num as i32;
                                    }
                                }
                                if self.terminal_str.starts_with("save ") && self.terminal_str.split(" ").count() == 2 {
                                    let level_name = self.terminal_str.split(" ").nth(1).unwrap().to_owned();
                                    self.current_level.title = level_name.clone();
                                    self.level_repository.save_level(level_name, self.name.clone(), self.current_level.clone());
                                }
                                if self.terminal_str.starts_with("list") && self.terminal_str.split(" ").count() == 1 {
                                    self.level_repository.print_levels();
                                }
                                if self.terminal_str.starts_with("rp") && self.terminal_str.split(" ").count() == 1 {
                                    self.completed_levels = HashSet::new();
                                }
                                if self.terminal_str.starts_with("dims ") && self.terminal_str.split(" ").count() == 3 {
                                    if let Ok(new_w) = self.terminal_str.split(" ").nth(1).unwrap().parse::<u32>() {
                                        if let Ok(new_h) = self.terminal_str.split(" ").nth(2).unwrap().parse::<u32>() {
                                            // change dims and copy over tiles
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
                                            // protect against stupid big
                                            // o shit oob entities
                                        }
                                    }
                                }

                                // if list
                                // if dims etc...


                                self.terminal_history.push(self.terminal_str.clone());
                                self.terminal_str = "".to_owned();
                            }
                        },
                        VirtualKeyCode::Back => {
                            if !self.terminal_str.is_empty() {
                                self.terminal_str.pop();
                            }
                        },
                        // yeah desired up/down arrow key behaviour looks like spaghetti but it is what you want intuitively
                        VirtualKeyCode::Up => {
                            if let Some(idx) = self.history_idx {
                                // already in history
                                self.history_idx = Some((idx + 1).min((self.terminal_history.len() - 1) as i32));
                                self.terminal_str = self.terminal_history[self.terminal_history.len() - 1 - self.history_idx.unwrap() as usize].clone();
                            } else {
                                if self.terminal_history.len() !=  0 {
                                    if !self.terminal_str.is_empty() {
                                        self.terminal_history.push(self.terminal_str.clone());
                                    }
                                    self.history_idx = Some(0.min((self.terminal_history.len() - 1) as i32));
                                    self.terminal_str = self.terminal_history[self.terminal_history.len() - 1 - self.history_idx.unwrap() as usize].clone();
                                }
                            }
                        }
                        VirtualKeyCode::Down => {
                            if let Some(idx) = self.history_idx {
                                if self.history_idx == Some(0) {
                                    self.history_idx = None;
                                    self.terminal_str = "".to_owned();
                                } else {
                                    self.history_idx = Some((idx - 1).max(0));
                                    self.terminal_str = self.terminal_history[self.terminal_history.len() - 1 - self.history_idx.unwrap() as usize].clone();
                                }
                            } else {
                                if !self.terminal_str.is_empty() {
                                    self.terminal_history.push(self.terminal_str.clone());
                                    self.terminal_str = "".to_owned();
                                }
                            }
                        }
                        _ => {},
                    }
                }
                let mut render_str = self.terminal_str.clone();
                if (inputs.t/ 2.0) % 1.0 > 0.5 {
                    render_str.push('_');
                };
                render_text_left(render_str.as_bytes(), term_rect.dilate_pc(-0.005), 2.5, rc);
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

            let right_pane = Rect::new(level_pane.right(), 0.0, level_pane.x, inputs.screen_rect.h);
            let play = right_pane.grid_child(0, 0, 1, 5);
            rc.push(RenderCommand::solid_rect(play, Vec4::new(0.3, 0.3, 0.3, 1.0), 1.5));
            
            let text_rect = play.dilate_pc(-0.3);
            render_text_center(b"play", text_rect, 3.0, rc);
            if play.contains(inputs.mouse_pos) && inputs.lmb == KeyStatus::JustPressed {
                self.current_instance = Some(Instance::new(self.current_level.instance()));
            }
            // edit mode
            // draw editing controls, modify current_level
            // if they save we save it to level repository etc



        }

    }
}