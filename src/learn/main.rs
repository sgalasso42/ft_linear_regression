extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use piston::event_loop::*;
use piston::input::*;

mod maths;
mod parsing;
mod game;
mod algo;

use crate::game::game::*;
use crate::parsing::parse::*;
use crate::parsing::args::*;

// BONUS IDEAS
// scatter plot & residual plot representation (https://www.youtube.com/watch?v=_cXuvTQl090&list=PLRqwX-V7Uu6bCN8LKrcMa6zF4FPtXyXYj&index=6)
// batch gradient descent

fn main() {
    let config: Config = Config::new();
    let dataset: Vec<Pos> = parse_file(&config);
    println!("Data file: {}", config.file);
    println!("Algorithm: {:?}", config.algo);
    let mut game: Game = Game::setup(dataset, 500.0);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut game.window) {
        if let Some(event) = e.render_args() {
            game.render(&event);
        }
        if let Some(_event) = e.update_args() {
            game.update(&config);
        }
    }
}
