pub fn max_f64(values: &Vec<f64>) -> f64 {
    let mut max: f64 = 0.0;
    for value in values.iter() {
        if *value > max {
            max = *value;
        }
    }
    return max;
}