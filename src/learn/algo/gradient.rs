use crate::game::game::*;

pub fn gradient_descent(game: &mut Game) {
    let learning_rate: f64 = 0.01;
    for data in game.dataset.iter() {
        let guess = game.m * data.x + game.b;
        let error = data.y - guess;
        game.m += (error * data.x) * learning_rate;
        game.b += (error) * learning_rate;
    }
}