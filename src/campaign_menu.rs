use glutin::event::VirtualKeyCode;

use crate::world::*;
use crate::instance::*;
use crate::lib::kinput::*;
use crate::lib::kmath::*;
use crate::renderer::*;
pub struct CampaignMenu {
    world_selection: i32,
    level_selection: i32,
    game_instance: Option<Instance>,
    worlds: Vec<World>,

}

impl CampaignMenu {
    pub fn new() -> CampaignMenu {
        CampaignMenu {
            game_instance: None,
            world_selection: 0,
            level_selection: 0,
            worlds: vec![
                World::world1(),
                World::world2(),
                World::world3(),
            ],
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) {

        if let Some(instance) = self.game_instance.as_mut() {
            if instance.frame(inputs, rc, 0) {
                // complete level
                self.worlds[self.world_selection as usize].completion[self.level_selection as usize] = true;
                if self.level_selection as usize == self.worlds[self.world_selection as usize].levels.len() - 1 {
                    // last level in world
                    self.level_selection = 0;
                    self.game_instance = None;
                } else {
                    self.level_selection += 1;
                    self.game_instance = Some(Instance {
                        level: self.worlds[self.world_selection as usize].levels[self.level_selection as usize].clone().instance(),
                    });
                }
            }
        } else {
            if inputs.just_pressed(VirtualKeyCode::W) || inputs.just_pressed(VirtualKeyCode::Up) {
                self.world_selection = (self.world_selection - 1).max(0);
            }
            if inputs.just_pressed(VirtualKeyCode::S) || inputs.just_pressed(VirtualKeyCode::Down) {
                self.world_selection = (self.world_selection + 1).min(self.worlds.len() as i32 - 1);
            }


            // Draw
            let menu_rect = inputs.screen_rect.fit_center_square();
            for i in 0..self.worlds.len() {
                let world_rect = menu_rect.grid_child(0, i as i32, 1, self.worlds.len() as i32).dilate_pc(-0.1);
                rc.push(RenderCommand::solid_rect(world_rect, Vec4::new(0.3, 0.3, 0.3, 1.0), 1.0));

                let text_rect = world_rect.dilate_pc(-0.3);

                // draw world name
                render_text_center(&self.worlds[i].title.as_bytes(), text_rect, 3.0, rc);
                
                if self.world_selection == i as i32 {
                    rc.push(RenderCommand::solid_rect(world_rect, Vec4::new(1.0, 1.0, 0.0, 1.0), 1.5));
                    if inputs.just_pressed(VirtualKeyCode::Space) || inputs.just_pressed(VirtualKeyCode::Return) {
                        self.game_instance = Some(Instance {
                            level: self.worlds[self.world_selection as usize].levels[self.level_selection as usize].clone().instance(),
                        });
                    }
                }
                rc.push(RenderCommand::solid_rect(world_rect.dilate_pc(-0.05), Vec4::new(0.8, 0.8, 0.8, 1.0), 2.0));
            }
        }
    }
}