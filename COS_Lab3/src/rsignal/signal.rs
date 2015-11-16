
pub const PI: f32 = ::std::f32::consts::PI;
pub const N: u32 = 1024;
const APS: [u8; 7] = [1, 3, 5, 8, 10, 12, 16];
const FIS: [f32; 6] = [PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, 3.0 * PI / 4.0, PI];

pub fn poly_signal(ai: usize, fi: usize, n: f32, j: Option<f32>) -> f32 {
    let j = if let Some(v) = j { v } else { 1.0 };
    APS[ai] as f32 * ((2.0 * PI * n * j / N as f32) - FIS[fi]).cos()
}

pub fn signal(a: f32, fi: f32, n: f32, j: Option<f32>) -> f32 {
    let j = if let Some(v) = j { v } else { 1.0 };
    a * ((2.0 * PI * n * j / N as f32) - fi).cos()
}

pub fn a_sin_j(seq: &[f32], sins: &[f32], j: usize) -> f32 {
    let mut sum: f32 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        sum += y * sins[i * j];
    }
    2.0 * sum / N as f32
}

pub fn a_cos_j(seq: &[f32], sins: &[f32], j: usize) -> f32 {
    let mut sum: f32 = 0.0;
    for (i, y) in seq.iter().enumerate() {
        sum += y * sins[(i * j + (N as f32 / 4.0) as usize) % N as usize];
    }
    2.0 * sum / N as f32
}

pub fn aj(seq: &[f32], sins: &[f32], j: usize) -> f32 {
    let acj = a_cos_j(seq, sins, j);
    let asj = a_sin_j(seq, sins, j);
    (acj.powi(2) + asj.powi(2)).sqrt()
}

pub fn fi(seq: &[f32], sins: &[f32], j: usize) -> f32 {
    let acj = a_cos_j(seq, sins, j);
    let asj = a_sin_j(seq, sins, j);
    (acj / asj).atan()
}

