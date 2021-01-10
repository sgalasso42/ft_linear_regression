use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent};
use piston::window::WindowSettings;

extern crate ft_linear_regression;
use ft_linear_regression::parsing::load_file::*;
use ft_linear_regression::graphics::render::*;

// BONUS IDEAS
// scatter plot & residual plot representation (https://www.youtube.com/watch?v=_cXuvTQl090&list=PLRqwX-V7Uu6bCN8LKrcMa6zF4FPtXyXYj&index=6)
// linear regression with ordinary least squares method

fn main() {
    let datas: Vec<Pos> = load_file();
    let size: f64 = 500.0;

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("ft_linear_regression", [size, size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut display = Display::new(opengl, size, 20.0);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            display.clear(&args, GREY1);

            // points
            for data in datas.iter() {
                display.render_ellipse(&args, data, 6.0, WHITE);
            }

            // [!] to protect : check if dataset.len() > 1 (because line need two points)
            // linear regression (ordinary least squares)
            let mut xsum: f64 = 0.0;
            let mut ysum: f64 = 0.0;
            for data in datas.iter() {
                xsum += data.x;
                ysum += data.y;
            }
            let x_average: f64 = xsum / datas.len() as f64;
            let y_average: f64 = ysum / datas.len() as f64;
            let mut num: f64 = 0.0;
            let mut den: f64 = 0.0;
            for data in datas.iter() {
                num += (data.x - x_average) * (data.y - y_average);
                den += (data.x - x_average) * (data.x - x_average);
            }
            let m: f64 = num / den; // [!] to protect : can be 0 if all x values are the same
            let b: f64 = y_average - m * x_average;

            // line
            let line_a: Pos = Pos::new(0.0, m * 0.0 + b);
            let line_b: Pos = Pos::new(1.0, m * 1.0 + b);
            display.render_line(&args, &line_a, &line_b, 1.0, WHITE);
        }
    }
}
