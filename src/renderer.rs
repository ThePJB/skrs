use crate::lib::kmath::*;
use crate::lib::kimg::*;

use glow::*;

use std::fmt;

pub const DEPTH_WORLD_START: f32 = 2.0;
pub const DEPTH_WORLD_END: f32 = 10.0;

#[derive(Clone, Copy, Debug)]
pub struct RenderCommand {
    pub colour: Vec4,
    pub pos: Rect,
    pub sprite_clip: Rect,
    pub depth: f32,
}


impl RenderCommand {
    pub fn solid_rect(r: Rect, colour: Vec4, depth: f32) -> RenderCommand {
        RenderCommand {
            colour,
            pos: r,
            sprite_clip: Rect::new(9.0, 9.0, 1.0, 1.0).grid_child(2, 0, 10, 10),
            depth,
        }
    }
}


pub struct Renderer {
    vbo: NativeBuffer,
    vao: NativeVertexArray,
    shader: NativeProgram,

    atlas: NativeTexture,

}

impl Renderer {
    pub fn new(gl: &glow::Context, shader: NativeProgram, atlas: ImageBufferA) -> Renderer {
        
        unsafe {
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, atlas.w as i32, atlas.h as i32, 0, RGBA, glow::UNSIGNED_BYTE, Some(&atlas.bytes()));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.generate_mipmap(glow::TEXTURE_2D);

            // We construct a buffer and upload the data
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            // We now construct a vertex array to describe the format of the input buffer
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*4 + 4*3 + 4*2, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 4*4 + 4*3 + 4*2, 4*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 4*4 + 4*3 + 4*2, 4*4 + 4*3);
            gl.enable_vertex_attrib_array(2);
    
            Renderer {
                vao,
                vbo,
                shader,
                atlas: texture,
            }
        }
    }

    pub fn draw(&mut self, gl: &glow::Context, screen_rect: Rect, atlas_rect: Rect, commands: &[RenderCommand]) {

        // commands -> quads on screen -> verts -> floats -> bytes
        let buf: Vec<u8> = commands.iter()
            // cull
            .filter(|c| screen_rect.overlaps(c.pos).is_some())

            // screen -> ndc
            .map(|c| {
                let ndc_pos = c.pos.transform(screen_rect, Rect::new(0.0, 0.0, 1.0, 1.0));
                RenderCommand {
                    colour: c.colour,
                    pos: ndc_pos,
                    sprite_clip: c.sprite_clip,
                    depth: c.depth

                }
            })

            // screen command to (vec3, vec4, vec2)[4]
            .map(|c| {
                // assuming gpu wants UVs to be unit rect
                let clip_rect = c.sprite_clip.transform(atlas_rect, Rect::new(0.0, 0.0, 1.0, 1.0));

                [
                    (c.pos.tl().promote(c.depth), c.colour, clip_rect.tl()),
                    (c.pos.tr().promote(c.depth), c.colour, clip_rect.tr()),
                    (c.pos.br().promote(c.depth), c.colour, clip_rect.br()),
                    (c.pos.bl().promote(c.depth), c.colour, clip_rect.bl()),
                ]
            })

            // q[4] -> q[6]
            .map(|q| [
                q[0], q[1], q[3], q[1], q[2], q[3],
            ])
            .flatten()

            // q -> float[9]
            .map(|(pos, colour, uv)| [
                pos.x,
                pos.y,
                pos.z,
                colour.x,
                colour.y,
                colour.z,
                colour.w,
                uv.x,
                uv.y
            ])
            .flatten()
            // float -> bytes
            .map(|f| f.to_le_bytes())
            .flatten()
            .collect();

            unsafe {
                gl.use_program(Some(self.shader));
                gl.bind_texture(glow::TEXTURE_2D, Some(self.atlas));
                gl.bind_vertex_array(Some(self.vao));
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buf, glow::DYNAMIC_DRAW);
                let vert_count = buf.len() / (7*4);
                gl.draw_arrays(glow::TRIANGLES, 0, vert_count as i32);
            }
    
    }
    
    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_vertex_array(self.vao);
            gl.delete_texture(self.atlas);
        }
    }
}