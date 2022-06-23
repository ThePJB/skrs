use glutin::event::VirtualKeyCode;

use crate::level::*;
use crate::lib::kinput::*;
use crate::manifest::noice_levels;
use crate::renderer::*;
use crate::lib::kmath::*;

// gameplay instance
pub struct Instance {
    pub level: Level,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            level: Level::from_string(noice_levels[0]).unwrap(),
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) -> bool {
        if !self.level.victorious() {
            if inputs.just_pressed(VirtualKeyCode::W) || inputs.just_pressed(VirtualKeyCode::Up) {
                self.level.try_move((0, -1));
            }
            if inputs.just_pressed(VirtualKeyCode::S) || inputs.just_pressed(VirtualKeyCode::Down) {
                self.level.try_move((0, 1));
            }
            if inputs.just_pressed(VirtualKeyCode::A) || inputs.just_pressed(VirtualKeyCode::Left) {
                self.level.try_move((-1, 0));
            }
            if inputs.just_pressed(VirtualKeyCode::D) || inputs.just_pressed(VirtualKeyCode::Right) {
                self.level.try_move((1, 0));
            }
            if inputs.just_pressed(VirtualKeyCode::Z) {
                self.level.undo();
            }
            if self.level.victorious() {
                println!("winner!")
            }
        }
        if inputs.just_pressed(VirtualKeyCode::Space) || inputs.just_pressed(VirtualKeyCode::Return) {
            if self.level.victorious() {
                return true;
            } else {
                std::process::exit(0);
            }
        }

        // draw level
        let level_rect = inputs.screen_rect.fit_aspect_ratio(self.level.w as f32 / self.level.h as f32);
        for i in 0..self.level.w {
            for j in 0..self.level.h {
                rc.push(RenderCommand {
                    colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    sprite_clip: match self.level.tiles[(j*self.level.w + i) as usize] {
                        Tile::Snow => Rect::new(1.0, 0.0, 1.0, 1.0),
                        Tile::Ice => Rect::new(2.0, 0.0, 1.0, 1.0),
                        Tile::Wall => Rect::new(0.0, 0.0, 1.0, 1.0),
                    },
                    pos: level_rect.grid_child(i, j, self.level.w, self.level.h),
                    depth: 1.0,
                });
            }
        }
        for (e, i, j) in self.level.entities.iter() {
            rc.push(RenderCommand {
                colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
                sprite_clip: match e {
                    Entity::Player => Rect::new(4.0, 0.0, 1.0, 1.0),
                    Entity::Present => Rect::new(3.0, 0.0, 1.0, 1.0),
                    Entity::Receptacle => Rect::new(5.0, 0.0, 1.0, 1.0),
                },
                pos: level_rect.grid_child(*i, *j, self.level.w, self.level.h),
                depth: match e {
                    Entity::Player | Entity::Present => 2.0,
                    Entity::Receptacle => 1.5,
                },
            });
        }

        return false;
    }
}