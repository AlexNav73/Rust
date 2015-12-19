
const N: usize = 512;
const PI: f64 = ::std::f64::consts::PI;

fn a_sin_j(seq: &[f64], j: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        sum += y * (2.0 * PI * i as f64 * j as f64 / N as f64).sin();
    }
    2.0 * sum / N as f64
}

fn a_cos_j(seq: &[f64], j: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        sum += y * (2.0 * PI * i as f64 * j as f64 / N as f64).cos();
    }
    2.0 * sum / N as f64
}

pub fn aj(seq: &[f64], j: usize) -> f64 {
    let acj = a_cos_j(seq, j);
    let asj = a_sin_j(seq, j);
    (acj.powi(2) + asj.powi(2)).sqrt()
}

pub fn fi(seq: &[f64], j: usize) -> f64 {
    let acj = a_cos_j(seq, j);
    let asj = a_sin_j(seq, j);
    (asj / acj).atan()
}

