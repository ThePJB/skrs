use glutin::event::VirtualKeyCode;

use crate::level::*;
use crate::lib::kinput::*;
use crate::renderer::*;
use crate::lib::kmath::*;

// gameplay instance
pub struct Instance {
    pub level: LevelInstance,
}

#[derive(Debug)]
pub enum InstanceFrameOutcome {
    Completion(String),
    Travel(String),
    Bail,
    None,
}

impl Instance {
    pub fn new(level_instance: LevelInstance) -> Instance {
        Instance {
            level: level_instance,
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>, num_tokens: i32, t: f32) -> InstanceFrameOutcome {
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

        // draw level
        let level_rect = inputs.screen_rect.fit_aspect_ratio(self.level.l.w as f32 / self.level.l.h as f32);
        self.level.render(level_rect, rc, num_tokens, t);
        
        // Handle possible outcomes
        for (e1, i1, j1) in self.level.current_entities.iter() {
            for (e2, i2, j2) in self.level.current_entities.iter() {
                if *i1 == *i2 && *j1 == *j2 {
                    match (e1, e2) {
                        (Entity::Player, Entity::Portal(tokens, dest)) => {
                            if num_tokens >= *tokens {
                                return InstanceFrameOutcome::Travel(dest.clone())
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
        if self.level.victorious() {
            return InstanceFrameOutcome::Completion(self.level.l.title.clone());
        }
        if inputs.just_pressed(VirtualKeyCode::Escape) {
            return InstanceFrameOutcome::Bail;
        }
        return InstanceFrameOutcome::None;
    }
}