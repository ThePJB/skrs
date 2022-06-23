use crate::lib::kinput::*;
use crate::lib::kmath::*;
use crate::renderer::*;


use glutin::event::VirtualKeyCode;

const width: i32 = 10;
const height: i32 = 10;

pub struct Game {
    w: i32,
    h: i32,
    snake: Vec<(i32, i32)>,
    food: (i32, i32),
    direction: (i32, i32),

    t: f64,
    steps: i32,
    dead: bool,
}

impl Game {
    pub fn new() -> Game {
        Game{
            w: width,
            h: height,
            snake: vec![(width/2, height/2), (width/2 - 1, height/2)],
            food: (3, 3),
            t: 0.0,
            steps: 0,
            dead: false,
            direction: (1, 0),
        }
    }

    // returns cam rect
    pub fn frame(&mut self, inputs: &FrameInputState, rc: &mut Vec<RenderCommand>) -> Rect {
     
        // rc.push(RenderCommand::screen_rect(game_rect, Vec4::new(0.3, 0.3, 0.3, 1.0), 1.0));
        // ok maybe glteximg2d needs to not be transposed
        rc.push(RenderCommand { 
            colour: Vec4::new(1.0, 1.0, 1.0, 1.0), 
            pos: inputs.screen_rect,
            sprite_clip: Rect::new(1.0, 0.0, 1.0, 1.0),
            depth: 1.0,
        });

        // calculation of camera: aspect ratio conforms to window, fixed height
        return Rect::new(0.0, 0.0, 1.0, 1.0);

    }
}