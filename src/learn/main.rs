use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent};
use piston::window::WindowSettings;

mod maths;
mod parsing;
mod graphics;

use crate::graphics::render::*;
use crate::parsing::load_file::*;

// BONUS
// visualisator
// linear regression with ordinary least squares method

// BONUS IDEAS
// scatter plot & residual plot representation (https://www.youtube.com/watch?v=_cXuvTQl090&list=PLRqwX-V7Uu6bCN8LKrcMa6zF4FPtXyXYj&index=6)
// batch gradient descent

fn ordinary_least_squares(dataset: &Vec<Pos>, m: &mut f64, b: &mut f64) {
    // [!] to protect : check if dataset.len() > 1 (because line need two points)
    let mut xsum: f64 = 0.0;
    let mut ysum: f64 = 0.0;
    for data in dataset.iter() {
        xsum += data.x;
        ysum += data.y;
    }
    let x_average: f64 = xsum / dataset.len() as f64;
    let y_average: f64 = ysum / dataset.len() as f64;
    let mut num: f64 = 0.0;
    let mut den: f64 = 0.0;
    for data in dataset.iter() {
        num += (data.x - x_average) * (data.y - y_average);
        den += (data.x - x_average) * (data.x - x_average);
    }
    *m = num / den; // [!] to protect : can be 0 if all x values are the same
    *b = y_average - *m * x_average;
}

fn gradient_descent(dataset: &Vec<Pos>, m: &mut f64, b: &mut f64) {
    let learning_rate: f64 = 0.01;
    for data in dataset.iter() {
        let guess = *m * data.x + *b;
        let error = data.y - guess;
        *m += (error * data.x) * learning_rate;
        *b += (error) * learning_rate;
    }
}

fn main() {
    let (dataset, algo, filename): (Vec<Pos>, Algo, String) = load_file();
    println!("Data file used: {}", filename);
    println!("Chosen algorithm: {:?}", algo);

    let size: f64 = 500.0;

    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("ft_linear_regression", [size, size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut display = Display::new(opengl, size, 20.0);
    let mut events = Events::new(EventSettings::new());

    let mut m: f64 = 0.0;
    let mut b: f64 = 0.0;

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            display.clear(&args, GREY1);

            // points display
            for data in dataset.iter() {
                display.render_ellipse(&args, data, 6.0, WHITE);
            }

            // linear regression
            match &algo {
                Algo::Ols => ordinary_least_squares(&dataset, &mut m, &mut b),
                Algo::Gradient => gradient_descent(&dataset, &mut m, &mut b)
            }
            
            // line display
            let line_a: Pos = Pos::new(0.0, m * 0.0 + b);
            let line_b: Pos = Pos::new(1.0, m * 1.0 + b);
            display.render_line(&args, &line_a, &line_b, 1.0, WHITE);
        }
    }
}
