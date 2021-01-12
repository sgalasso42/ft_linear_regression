extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::*;
use piston::input::*;

mod maths;
mod parsing;
mod game;
mod algo;
mod output;

use crate::game::game::*;
use crate::parsing::parse::*;
use crate::parsing::args::*;
use crate::output::export::*;

fn main() {
    let config: Config = Config::new();
    let (dataset, max_values): (Vec<Pos>, Pos) = parse_file(&config);
    println!("Data file: {}", config.file);
    println!("Algorithm: {:?}", config.algo);
    let mut game: Game = Game::setup(&config.algo, dataset, 600.0, 600.0);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut game.window) {
        if let Some(event) = e.render_args() {
            game.gl.draw(event.viewport(), |_context, gl| {
                graphics::clear(GREY1, gl);
            });
            game.render_scatter_plot(&event);
            game.render_residual_plot(&event);
            game.render_sq_residuals_sum_plot(&event);
            game.render_separators(&event);
        }
        if let Some(_event) = e.update_args() {
            game.update(&config);
        }
    }

    export_to_file(&game, max_values);
}