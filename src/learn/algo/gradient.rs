use crate::game::game::*;
use crate::output::export::*;

pub fn gradient_descent(game: &mut Game) {
    let learning_rate: f64 = 0.01;
    if game.step_nb < 1000 {
        game.step_nb += 1;
        let (srs_b_d, srs_m_d): (f64, f64) = calculate_srs_derivative(game);
        let b_step_size: f64 = srs_b_d * learning_rate;
        let m_step_size: f64 = srs_m_d * learning_rate;
        if b_step_size.abs() <= 0.001 && m_step_size.abs() <= 0.001 {
            export_to_file(&game);
            game.linear_regression_finshed = true;
        }
        game.b -= b_step_size;
        game.m -= m_step_size;
    }
}

fn calculate_srs_derivative(game: &mut Game) -> (f64, f64) {
    let mut srs_b_d: f64 = 0.0;
    let mut srs_m_d: f64 = 0.0;
    let mut srs_b: f64 = 0.0;
    let mut srs_m: f64 = 0.0;
    for data in game.dataset.iter() {
        let guess = game.b + game.m * data.x;
        let error = data.y - guess;
        srs_b += error * error;
        srs_m += (data.x * error) * (data.x * error);
        srs_b_d += -2.0 * error;
        srs_m_d += -2.0 * data.x * error;
    }
    game.srs_b_list.push(srs_b);
    game.srs_m_list.push(srs_m);
    return (srs_b_d, srs_m_d);
}