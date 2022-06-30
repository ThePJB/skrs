use glutin::event::VirtualKeyCode;

use crate::level::*;
use crate::lib::kinput::*;
use crate::manifest::noice_levels;
use crate::renderer::*;
use crate::lib::kmath::*;

// gameplay instance
pub struct Instance {
    pub level: LevelInstance,
}

impl Instance {
    pub fn new(level_instance: LevelInstance) -> Instance {
        Instance {
            level: level_instance,
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
            }
        }
        if inputs.just_pressed(VirtualKeyCode::Escape) {
            return true;
        }

        // draw level
        let level_rect = inputs.screen_rect.fit_aspect_ratio(self.level.l.w as f32 / self.level.l.h as f32);
        self.level.render(level_rect, rc);

        return false;
    }
}