
use gens::*;
use histogram::*;

fn process<'a, G>(gen: &'a G, count: u32, 
    max: Option<f64>,
    min: Option<f64>,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64),
    copycallback: fn(f64)) where &'a G: traits::IntoGenerator<Output=f64> {

    let mut hist = Histogram::new();
    hist.fill(&gen, count);

    let period = hist.initialize();
    if let Some(mx) = max {
        if let Some(mn) = min {
            hist.count_vals_occurance(mx, mn);
        }
    } else {
        let max = hist.max();
        let min = hist.min(max);
        hist.count_vals_occurance(max, min);
    }
    let result = hist.evenly_check();

    let math = hist.math();
    let disp = hist.disp(math);
    let aver_disp = Histogram::msquare(disp);

    hist.retrive_occurance(callback);
    hist.retrive_sequance(copycallback);
    mcallback(period as u32, result, math, disp, aver_disp);
}

/// 
/// callback: Retrive value from period list to c#
/// mcallback: Retrive math waiting, disperssion, average square dispersion 
/// copycallback: Send all values in sequance to c#
///
#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateSimpsonGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64, v2: f64,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64), 
    copycallback: fn(f64)) {

    let gen = simpson::Generator::new(a as f64, r0, m as f64, v1, v2);
    process(&gen, count, Some(v1), Some(v2), callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateExpGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64,
    callback: fn(f64), 
    mcallback: fn(u32, f64, f64, f64, f64), 
    copycallback: fn(f64)) {

    let gen = exp::Generator::new(a as f64, r0, m as f64, v1);
    process(&gen, count, None, None, callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateUniformGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64, v2: f64,
    callback: fn(f64), 
    mcallback: fn(u32, f64, f64, f64, f64), 
    copycallback: fn(f64)) {

    let gen = uniform::Generator::new(a as f64, r0, m as f64, v1, v2);
    process(&gen, count, Some(v1), Some(v2), callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateGausGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64, v2: f64,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64), 
    copycallback: fn(f64)) {

    let gen = gaus::Generator::new(a as f64, r0, m as f64, v1, v2);
    process(&gen, count, None, None, callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateLemerGenerator(
    count: u32, a: i32, r0: f64, m: i32,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64),
    copycallback: fn(f64)) {

    let gen = lemer::Generator::new(a as f64, r0, m as f64);
    process(&gen, count, Some(1_f64), Some(0_f64), callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateTriangularGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64, v2: f64,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64),
    copycallback: fn(f64)) {

    let gen = triangular::Generator::new(a as f64, r0, m as f64, v1, v2);
    process(&gen, count, None, None, callback, mcallback, copycallback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateGammaGenerator(
    count: u32, a: i32, r0: f64, m: i32, v1: f64, v2: f64,
    callback: fn(f64),
    mcallback: fn(u32, f64, f64, f64, f64),
    copycallback: fn(f64)) {

    let gen = gamma::Generator::new(a as f64, r0, m as f64, v1, v2);
    process(&gen, count, None, None, callback, mcallback, copycallback);
}
