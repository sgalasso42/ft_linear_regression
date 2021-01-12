use crate::game::game::*;

pub fn gradient_descent(game: &mut Game) {
    let learning_rate: f64 = 0.2;
    let (msum, bsum, squared_residual): (f64, f64, f64) = error_sum(game);
    let m_tmp = (learning_rate * msum) / game.dataset.len() as f64;
    let b_tmp = (learning_rate * bsum) / game.dataset.len() as f64;
    game.srsum_list.push(squared_residual);
    game.m += m_tmp;
    game.b += b_tmp;
}

fn error_sum(game: &Game) -> (f64, f64, f64) {
    let mut msum: f64 = 0.0;
    let mut bsum: f64 = 0.0;
    let mut squared_residual: f64 = 0.0;
    for data in game.dataset.iter() {
        let guess = game.b + game.m * data.x;
        let error = data.y - guess;
        squared_residual += error * error;
        msum += error * data.x;
        bsum += error;
    }
    return (msum, bsum, squared_residual);
}