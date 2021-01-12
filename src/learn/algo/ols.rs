use crate::game::game::*;
use crate::output::export::*;

pub fn ordinary_least_squares(game: &mut Game) {
    let mut xsum: f64 = 0.0;
    let mut ysum: f64 = 0.0;
    for data in game.dataset.iter() {
        xsum += data.x;
        ysum += data.y;
    }
    let x_average: f64 = xsum / game.dataset.len() as f64;
    let y_average: f64 = ysum / game.dataset.len() as f64;
    let mut num: f64 = 0.0;
    let mut den: f64 = 0.0;
    for data in game.dataset.iter() {
        num += (data.x - x_average) * (data.y - y_average);
        den += (data.x - x_average) * (data.x - x_average);
    }
    game.m = num / den;
    game.b = y_average - game.m * x_average;
    game.linear_regression_finshed = true;
    export_to_file(&game);
}