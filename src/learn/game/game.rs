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
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct Space {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Space {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64) -> Self {
        return Space {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1
        }
    }
}

#[derive(Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        return Pos {
            x: x,
            y: y
        }
    }

    pub fn scale(&self, space: &Space) -> Self {
        return Pos {
            x: scale(self.x, 0.0, 1.0, space.x0, space.x1),
            y: scale(self.y, 0.0, 1.0, space.y0, space.y1)
        }
    }
}

pub struct Game {
    pub window_space: Space,
    pub splot_space: Space,
    pub rplot_space: Space,
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub dataset: Vec<Pos>,
    pub m: f64,
    pub b: f64
}

impl Game {
    pub fn setup(dataset: Vec<Pos>, window_w: f64, window_h: f64) -> Self {
        let opengl: OpenGL = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("learn", [window_w, window_h])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .expect("error: can't initialize the GlutinWindow");
        return Game {
            window_space: Space::new(0.0, window_w, window_h, 0.0),
            splot_space: Space::new(0.0, window_w / 2.0, window_h / 2.0, 0.0),
            rplot_space: Space::new(0.0, window_w, window_h, window_h / 2.0),
            window: window,
            gl: GlGraphics::new(opengl),
            dataset: dataset,
            m: 0.0,
            b: 0.0
        };
    }

    pub fn render_separators(&mut self, event: &RenderArgs) {
        // render vertical separator
        let line_size: f64 = 5.0;
        let pos0: Pos = Pos::new(1.0, 0.0).scale(&self.splot_space);
        let pos1: Pos = Pos::new(1.0, 1.0).scale(&self.splot_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(BLACK, line_size, [
                pos0.x, pos0.y,
                pos1.x, pos1.y],
            c.transform, gl);
        });

        // render horizontal separator
        let line_size: f64 = 5.0;
        let pos0: Pos = Pos::new(0.0, 1.0).scale(&self.rplot_space);
        let pos1: Pos = Pos::new(1.0, 1.0).scale(&self.rplot_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(BLACK, line_size, [
                pos0.x, pos0.y,
                pos1.x, pos1.y],
            c.transform, gl);
        });
    }

    pub fn render_scatter_plot(&mut self, event: &RenderArgs) {
        // render data points
        let ellipse_size: f64 = 6.0;
        let squares: Vec<graphics::types::Rectangle> = self.dataset.iter().map(|pos| {
            let scaled_pos: Pos = pos.scale(&self.splot_space);
            return graphics::rectangle::square(
                scaled_pos.x - ellipse_size / 2.0,
                scaled_pos.y - ellipse_size / 2.0,
                ellipse_size);
        }).collect();
        self.gl.draw(event.viewport(), |context, gl| {
            squares.into_iter().for_each(|square| graphics::ellipse(WHITE, square, context.transform, gl));
        });

        // render regression line
        let line_size: f64 = 1.0;
        let pos0: Pos = Pos::new(0.0, self.b).scale(&self.splot_space);
        let pos1: Pos = Pos::new(1.0, self.m * 1.0 + self.b).scale(&self.splot_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(WHITE, line_size, [
                pos0.x, pos0.y,
                pos1.x, pos1.y],
            c.transform, gl);
        });
    }

    pub fn render_residual_plot(&mut self, event: &RenderArgs) {  
        // render x axis
        let line_size: f64 = 1.0;
        let pos0: Pos = Pos::new(0.0, 0.5).scale(&self.rplot_space);
        let pos1: Pos = Pos::new(1.0, 0.5).scale(&self.rplot_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(WHITE, line_size, [
                pos0.x, pos0.y,
                pos1.x, pos1.y],
            c.transform, gl);
        });

        // render residuals
        let line_size: f64 = 1.0;
        let lines: Vec<[f64; 4]> = self.dataset.iter().map(|pos| {
            let calc_pos0: Pos = Pos::new(pos.x, 0.5 + pos.y - (self.b + self.m * pos.x));
            let calc_pos1: Pos = Pos::new(pos.x, 0.5);
            let scaled_pos0: Pos = calc_pos0.scale(&self.rplot_space);
            let scaled_pos1: Pos = calc_pos1.scale(&self.rplot_space);
            return [
                scaled_pos0.x, scaled_pos0.y,
                scaled_pos1.x, scaled_pos1.y
            ];
        }).collect();
        self.gl.draw(event.viewport(), |context, gl| {
            lines.into_iter().for_each(|line| {
                graphics::line(WHITE, line_size, line, context.transform, gl);
            });
        });
    }

    pub fn update(&mut self, config: &Config) {
        match config.algo {
            Algo::Ols => ordinary_least_squares(self),
            Algo::Gradient => gradient_descent(self)
        }
    }
}