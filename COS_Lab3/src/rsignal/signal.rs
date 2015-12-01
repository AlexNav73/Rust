
pub const PI: f64 = ::std::f64::consts::PI;
pub const N: usize = 512;

pub fn signal(a: f64, fi: f64, n: f64, j: f64) -> f64 {
    a * ((2.0 * PI * n * j / N as f64) - fi).cos()
}

pub fn a_sin_j(seq: &[f64], sins: &[f64], j: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        //sum += y * (2.0 * PI * i as f64 * j as f64 / N as f64).sin();
        sum += y * sins[(i * j) % N];
    }
    2.0 * sum / N as f64
}

pub fn a_cos_j(seq: &[f64], sins: &[f64], j: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        //sum += y * (2.0 * PI * i as f64 * j as f64 / N as f64).cos();
        sum += y * sins[(i * j + (N as f64 / 4.0) as usize) % N];
    }
    2.0 * sum / N as f64
}

pub fn aj(seq: &[f64], sins: &[f64], j: usize) -> f64 {
    let acj = a_cos_j(seq, sins, j);
    let asj = a_sin_j(seq, sins, j);
    (acj.powi(2) + asj.powi(2)).sqrt()
}

pub fn fi(seq: &[f64], sins: &[f64], j: usize) -> f64 {
    let acj = a_cos_j(seq, sins, j);
    let asj = a_sin_j(seq, sins, j);
    (asj / acj).atan()
}

