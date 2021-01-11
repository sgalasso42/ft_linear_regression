extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use crate::parsing::args::*;
use crate::maths::scale::*;
use crate::algo::gradient::*;
use crate::algo::ols::*;

pub const GREY1: [f32; 4] = [0.11, 0.11, 0.11, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

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

pub struct Game {
    pub size: f64,
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub dataset: Vec<Pos>,
    pub m: f64,
    pub b: f64
}

impl Game {
    pub fn setup(dataset: Vec<Pos>, window_size: f64) -> Self {
        let opengl: OpenGL = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("learn", [window_size, window_size])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .expect("error: can't initialize the GlutinWindow");
        return Game {
            size: window_size,
            window: window,
            gl: GlGraphics::new(opengl),
            dataset: dataset,
            m: 0.0,
            b: 0.0
        }; 
    }

    pub fn render(&mut self, event: &RenderArgs) {
        self.gl.draw(event.viewport(), |_context, gl| {
            graphics::clear(GREY1, gl);
        });
        
        let ellipse_size: f64 = 6.0;
        let squares: Vec<graphics::types::Rectangle> = self.dataset.iter().map(|pos| {
            graphics::rectangle::square(
                scale(pos.x, 0.0, 1.0, 0.0, self.size) - ellipse_size / 2.0,
                scale(pos.y, 0.0, 1.0, self.size, 0.0) - ellipse_size / 2.0,
                ellipse_size)
        }).collect();
        self.gl.draw(event.viewport(), |context, gl| {
            squares.into_iter().for_each(|square| graphics::ellipse(WHITE, square, context.transform, gl));
        });

        let line_thickness: f64 = 1.0;
        let pos0: Pos = Pos::new(0.0, self.m * 0.0 + self.b);
        let pos1: Pos = Pos::new(1.0, self.m * 1.0 + self.b);
        let scaled_pos0: Pos = Pos {
            x: scale(pos0.x, 0.0, 1.0, 0.0, self.size) - line_thickness / 2.0,
            y: scale(pos0.y, 0.0, 1.0, self.size, 0.0) - line_thickness / 2.0
        };
        let scaled_pos1: Pos = Pos {
            x: scale(pos1.x, 0.0, 1.0, 0.0, self.size) - line_thickness / 2.0,
            y: scale(pos1.y, 0.0, 1.0, self.size, 0.0) - line_thickness / 2.0
        };
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(WHITE, line_thickness, [scaled_pos0.x, scaled_pos0.y, scaled_pos1.x, scaled_pos1.y], c.transform, gl);
        });
    }

    pub fn update(&mut self, config: &Config) {
        match config.algo {
            Algo::Ols => ordinary_least_squares(self),
            Algo::Gradient => gradient_descent(self)
        }
    }
}