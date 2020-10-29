extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

/* Display -------------------------------------------------------- */

struct Display {
    gl: GlGraphics,
}

impl Display {
    fn new(opengl: OpenGL) -> Display {
        return Display {
            gl: GlGraphics::new(opengl)
        }
    }

    fn clear(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });
    }

    fn render_point(&mut self, args: &RenderArgs, x: f64, y: f64, size: f64, color: [f32; 4]) {
        let square = graphics::rectangle::square(x, y, size);
    
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::ellipse(color, square, c.transform, gl);
        });
    }

    fn render_line(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::line(BLACK, 0.5, [0.0, 0.0, 500.0, 500.0], c.transform, gl);
        });
    }
}

/* Parsing -------------------------------------------------------- */

fn load_file(args: &[String]) -> Vec<(i32, i32)> { // TODO : real parsing and security
	if args.len() != 2 {
		panic!("error: bad args number")
	}
	let file = File::open(&args[1]).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut datas: Vec<(i32, i32)> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
        if i == 0 { continue }
        else if let Ok(content) = line {
            let values: Vec<&str> = content.split(",").collect();
            let km: i32 = values[0].parse().expect("error: bad character");
            let price: i32 = values[1].parse().expect("error: bad character");
            datas.push((km, price));
		}
	}
	return datas;
}

/* Main ----------------------------------------------------------- */

fn main() {
    let args: Vec<String> = env::args().collect();
    let datas: Vec<(i32, i32)> = load_file(&args);
    
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Perceptron", [500, 500]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
    let mut display = Display::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            display.clear(&args);
        }
    }
}
