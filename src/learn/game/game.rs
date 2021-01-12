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
use crate::maths::max_f64::*;
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
    pub sumplot_space: Space,
    pub separator_size: f64,
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub dataset: Vec<Pos>,
    pub srs_b_list: Vec<f64>,
    pub srs_m_list: Vec<f64>,
    pub m: f64,
    pub b: f64,
    pub step_nb: i32,
    pub linear_regression_finshed: bool
}

impl Game {
    pub fn setup(dataset: Vec<Pos>, window_w: f64, window_h: f64) -> Self {
        let opengl: OpenGL = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("learn", [window_w, window_h])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .expect("error: can't initialize the GlutinWindow");
            let separator_size = 5.0;
        return Game {
            separator_size: separator_size,
            window_space: Space::new(0.0, window_w, window_h, 0.0),
            splot_space: Space::new(0.0, window_w / 2.0 - separator_size, window_h / 2.0 - separator_size, 0.0),
            rplot_space: Space::new(0.0, window_w, window_h, window_h / 2.0 + separator_size),
            sumplot_space: Space::new(window_w / 2.0 + separator_size, window_w, window_h / 2.0 - separator_size, 0.0),
            window: window,
            gl: GlGraphics::new(opengl),
            dataset: dataset,
            srs_b_list: vec![0.0],
            srs_m_list: vec![0.0],
            m: 1.0,
            b: 0.0,
            step_nb: 0,
            linear_regression_finshed: false
        };
    }

    pub fn render_separators(&mut self, event: &RenderArgs) {
        let separator_size: f64 = self.separator_size;

        // render vertical separator
        let pos0: Pos = Pos::new(0.5, 0.5).scale(&self.window_space);
        let pos1: Pos = Pos::new(0.5, 1.0).scale(&self.window_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(BLACK, separator_size, [
                pos0.x, pos0.y,
                pos1.x, pos1.y],
            c.transform, gl);
        });

        // render horizontal separator
        let pos0: Pos = Pos::new(0.0, 0.5).scale(&self.window_space);
        let pos1: Pos = Pos::new(1.0, 0.5).scale(&self.window_space);
        self.gl.draw(event.viewport(), |c, gl| {
            graphics::line(BLACK, separator_size, [
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

    pub fn render_sq_residuals_sum_plot(&mut self, event: &RenderArgs) { 
        // render squared residuals sum of b points
        let ellipse_size: f64 = 5.0;
        let max_sum: f64 = max_f64(&self.srs_b_list);
        let squares: Vec<graphics::types::Rectangle> = self.srs_b_list.iter().enumerate().map(|(index, sum)| {
            let scaled_sum: f64 = scale(*sum, 0.0, max_sum, 0.0, 1.0);
            let scaled_x: f64 = scale(index as f64, 0.0, self.srs_b_list.len() as f64, 0.0, 1.0);
            let pos: Pos = Pos::new(scaled_x, scaled_sum);
            let scaled_pos: Pos = pos.scale(&self.sumplot_space);
            return graphics::rectangle::square(
                scaled_pos.x - ellipse_size / 2.0,
                scaled_pos.y - ellipse_size / 2.0,
                ellipse_size);
        }).collect();
        self.gl.draw(event.viewport(), |context, gl| {
            squares.into_iter().for_each(|square| graphics::ellipse(WHITE, square, context.transform, gl));
        });

        // render squared residuals sum of m points
        let ellipse_size: f64 = 5.0;
        let max_sum: f64 = max_f64(&self.srs_m_list);
        let squares: Vec<graphics::types::Rectangle> = self.srs_m_list.iter().enumerate().map(|(index, sum)| {
            let scaled_sum: f64 = scale(*sum, 0.0, max_sum, 0.0, 1.0);
            let scaled_x: f64 = scale(index as f64, 0.0, self.srs_m_list.len() as f64, 0.0, 1.0);
            let pos: Pos = Pos::new(scaled_x, scaled_sum);
            let scaled_pos: Pos = pos.scale(&self.sumplot_space);
            return graphics::rectangle::square(
                scaled_pos.x - ellipse_size / 2.0,
                scaled_pos.y - ellipse_size / 2.0,
                ellipse_size);
        }).collect();
        self.gl.draw(event.viewport(), |context, gl| {
            squares.into_iter().for_each(|square| graphics::ellipse(WHITE, square, context.transform, gl));
        });
    }

    pub fn update(&mut self, config: &Config) {
        if self.linear_regression_finshed == false {
            match config.algo {
                Algo::Ols => ordinary_least_squares(self),
                Algo::Gradient => gradient_descent(self)
            }
        }
    }
}