extern crate opengl_graphics;
extern crate piston;

use crate::maths::scale::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs};

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub const GREY1: [f32; 4] = [0.11, 0.11, 0.11, 1.0];

#[derive(Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Pos {
        return Pos {
            x: x,
            y: y
        }
    }
}

pub struct Display {
    pub gl: GlGraphics,
    pub size: f64,
    pub padding: f64
}

impl Display {
    pub fn new(opengl: OpenGL, size: f64, padding: f64) -> Display {
        return Display {
            gl: GlGraphics::new(opengl),
            size: size,
            padding: padding
        }
    }
    pub fn clear(&mut self, args: &RenderArgs, color: [f32; 4]) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(color, gl);
        });
    }
    pub fn render_ellipse(&mut self, args: &RenderArgs, pos: &Pos, size: f64, color: [f32; 4]) {
        let scaled_pos: Pos = Pos {
            x: scale(pos.x, 0.0, 1.0, 0.0, self.size),
            y: scale(pos.y, 0.0, 1.0, self.size, 0.0)
        };
        let square = graphics::rectangle::square(scaled_pos.x - size / 2.0, scaled_pos.y - size / 2.0, size);

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::ellipse(color, square, c.transform, gl);
        });
    }
    pub fn render_rectangle(&mut self, args: &RenderArgs, pos: &Pos, size: f64, color: [f32; 4]) {
        let scaled_pos: Pos = Pos {
            x: scale(pos.x, 0.0, 1.0, 0.0, self.size),
            y: scale(pos.y, 0.0, 1.0, self.size, 0.0)
        };
        let square = graphics::rectangle::square(scaled_pos.x - size / 2.0, scaled_pos.y - size / 2.0, size);

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::rectangle(color, square, c.transform, gl);
        });
    }
    pub fn render_line(&mut self, args: &RenderArgs, pos0: &Pos, pos1: &Pos, size: f64, color: [f32; 4]) {
        let scaled_pos0: Pos = Pos {
            x: scale(pos0.x, 0.0, 1.0, 0.0, self.size),
            y: scale(pos0.y, 0.0, 1.0, self.size, 0.0)
        };
        let scaled_pos1: Pos = Pos {
            x: scale(pos1.x, 0.0, 1.0, 0.0, self.size),
            y: scale(pos1.y, 0.0, 1.0, self.size, 0.0)
        };
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::line(color, size, [scaled_pos0.x - size / 2.0, scaled_pos0.y - size / 2.0, scaled_pos1.x - size / 2.0, scaled_pos1.y - size / 2.0], c.transform, gl);
        });
    }
}