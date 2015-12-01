
use ::std::cmp::Ordering;
use ::signal::*;

const WINDOW_SIZE: usize = 7;
const N: usize = 15;
const K: usize = 3;

fn aliase(sig: &[f64]) -> f64 {
    let last = sig.len() - 1;
    let res = 131.0 * sig[3] + 5.0 * sig[0] + 5.0 * sig[last] - 30.0 * sig[1] -
        30.0 * sig[last - 1] + 75.0 * sig[2] + 75.0 * sig[last - 2];
    res / 231.0
}

pub fn parabola_aliasing<C: Fn(f64)>(s: &Signal, callback: C) {
    let n = s.xs.len();
    let mut result = Vec::with_capacity(n);
    let m = (7.0f32 / 2.0) as usize;

    for _ in 0..n { result.push(0.0f64); }
    for i in 0..m { result[i] = s.xs[i]; }
    for i in m..(n-m) { result[i] = aliase(&s.xs[(i - m)..(i + m)]); }
    for i in (n-m)..n { result[i] = s.xs[i]; }
    for x in result.iter() { callback(*x); }
}

pub fn window_aliasing<C: Fn(f64)>(s: &Signal, callback: C) {
    let n = s.xs.len();
    let m = WINDOW_SIZE / 2;
    let mut result = Vec::with_capacity(n);
    for _ in 0..n { result.push(0.0f64); }

    for i in 0..m { result[i] = s.xs[i]; }
    for i in m..(n-m) {
        let mut sum = 0.0;
        for j in (i-m)..(i+m+1) {
            sum += s.xs[j];
        }
        result[i] = sum / WINDOW_SIZE as f64;
    }
    for i in (n-m)..n { result[i] = s.xs[i]; }
    for y in result.iter() { callback(*y); }
}

fn array_copy(src: &[f64], dest: &mut [f64]) {
    let len_src = src.len();
    unsafe { ::std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), len_src); }
}

fn array_sort(src: &mut [f64]) {
    src.sort_by(|&x, &y| {
        if x < y {
            Ordering::Less
        } else if x == y {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
}

pub fn median_aliasing<C: Fn(f64)>(s: &Signal, callback: C) {
    let len = s.xs.len();
    let m = N / 2;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len { result.push(0.0f64); }

    for i in 0..m { result[i] = s.xs[i]; }

    let mut tmp = [0.0f64; N];
    for i in m..(len - m) {
        array_copy(&s.xs[(i - m)..(i + m)], &mut tmp);
        array_sort(&mut tmp);
        let mut sum = 0.0;
        for j in K..(N-K) {
            sum += tmp[j];
        }
        result[i] = sum / (N - 2 * K) as f64;
    }
    for i in (len - m)..len { result[i] = s.xs[i]; }
    for x in result.iter() { callback(*x); }
}
