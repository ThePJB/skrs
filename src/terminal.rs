use glutin::event::VirtualKeyCode;

use crate::lib::kmath::*;
use crate::lib::kinput::*;
use crate::renderer::*;


pub struct Terminal {
    pub terminal_history: Vec<String>,
    pub terminal_lines: Vec<String>,    // history and prints
    pub terminal_str: String,
    pub history_idx: Option<i32>,   
}

// interface point
pub enum TerminalCommand {
    New(String),
    Load(String),
    Save,
    // rename, delete etc
    Play,
    Dims(u32, u32),

    List, // levels, yeah how am I gonna get feedback
        // maybe right pane is a terminal and font size is a bit smaller
        // need a better font too
        // and we need some aspect ratio / size constraints

    // tbh this works but is kinda ghetto, you could pop up a dialog on place
    Link(String),
    Tokens(u32),
    Reset,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            terminal_history: Vec::new(),
            terminal_lines: Vec::new(),
            terminal_str: "".to_owned(),
            history_idx: None,
        }
    }

    fn term_logic(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) -> Option<TerminalCommand> {
        let edit = |c, s: &mut String, h: &mut Option<i32>| {
            s.push(c);
            *h = None;
        };
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
                    self.terminal_history.push(self.terminal_str.clone());
                    let mut display_line = self.terminal_str.clone();
                    display_line.insert(0, '>');
                    self.terminal_lines.push(display_line);
                    let command = self.terminal_str.clone();
                    self.terminal_str = "".to_owned();

                    // feedback is gonna be important esp wrt overwriting
                    // save vs saveas vs overwrite
                    // play should autosave
                    if !command.is_empty() {
                        if command.starts_with("open ") && command.split(" ").count() == 2 {
                            let level_name = command.split(" ").nth(1).unwrap().to_owned();
                            return Some(TerminalCommand::Load(level_name));
                        } else if command.starts_with("link ") && command.split(" ").count() == 2 {
                            let arg = command.split(" ").nth(1).unwrap().to_owned();
                            return Some(TerminalCommand::Link(arg));
                        } else if command.starts_with("tokens ") && command.split(" ").count() == 2 {
                            let arg = command.split(" ").nth(1).unwrap().to_owned();
                            if let Ok(num) = arg.parse::<u32>() {
                                return Some(TerminalCommand::Tokens(num));
                            }
                        } else if command.starts_with("save") && command.split(" ").count() == 1 {
                            return Some(TerminalCommand::Save);
                        } else if command.starts_with("list") && command.split(" ").count() == 1 {
                            return Some(TerminalCommand::List);
                        } else if command.starts_with("play") && command.split(" ").count() == 1 {
                            return Some(TerminalCommand::Play);
                        } else if command.starts_with("reset") && command.split(" ").count() == 1 {
                            return Some(TerminalCommand::Reset);
                        } else if command.starts_with("dims ") && command.split(" ").count() == 3 {
                            if let Ok(new_w) = command.split(" ").nth(1).unwrap().parse::<u32>() {
                                if let Ok(new_h) = command.split(" ").nth(2).unwrap().parse::<u32>() {
                                    return Some(TerminalCommand::Dims(new_w, new_h));
                                }
                            }
                        } else {
                            self.tprint(format!("bad command: {}", command));
                        }
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

        return None;
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>, term_rect: Rect) -> Option<TerminalCommand> {
        let result = self.term_logic(inputs, rc);

        let num_lines = 30;
        while self.terminal_lines.len() < num_lines as usize {
            self.terminal_lines.push("".to_owned());
        }

        // rendering goes here
        rc.push(RenderCommand::solid_rect(term_rect, Vec4::new(0.2, 0.2, 0.2, 1.0), 2.0));

        for i in (0..num_lines - 1) {
            let line_rect = term_rect.grid_child(0, num_lines - i - 1, 1, num_lines);
            if let Some(line_str) = self.terminal_lines.get(self.terminal_lines.len() - i as usize) {
                render_text_left(line_str.as_bytes(), line_rect.dilate_pc(-0.01), 2.5, rc);
            }
        }
        let line_rect = term_rect.grid_child(0, num_lines - 1, 1, num_lines);
        
        let mut render_str = self.terminal_str.clone();
        render_str.insert(0, '>');
        if (inputs.t/ 2.0) % 1.0 > 0.5 {
            render_str.push('_');
        };
        render_text_left(render_str.as_bytes(), line_rect.dilate_pc(-0.01), 2.5, rc);
        
        return result;
    }

    pub fn tprint(&mut self, msg: String) {
        self.terminal_lines.push(msg)
    } 
}