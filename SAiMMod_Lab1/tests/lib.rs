
extern crate rsgen2;
use rsgen2::gen::*;
use rsgen2::histogram::*;

#[test]
fn math_test() {
    let gen = Generator::new(55_f32, 3_f32, 777_f32);
    let mut hist = Histogram::new(&gen, 1000001);
    let _ = hist.initialize();
    assert!(hist.math() < 0.6);
}

#[test]
fn evenly_check_test() {
    let gen = Generator::new(55_f32, 3_f32, 777_f32);
    let mut hist = Histogram::new(&gen, 1000001);
    let _ = hist.initialize();

    let check = hist.evenly_check();
    println!("{}", check);
    let val = ::std::f32::consts::PI / 4_f32;
    let pressigion = 0.07;
    assert!(check < val + pressigion && check > val - pressigion);
}
