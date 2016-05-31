
extern crate rsgen;
use rsgen::gens::*;
use rsgen::histogram::*;

fn print(_v: f64) {
    println!("{}", _v);
}

fn main() {
    let gen = gaus::Generator::new(55_f64, 3_f64, 777_f64, 13_f64, 81_f64);
    //let gen = exp::Generator::new(55_f64, 3_f64, 777_f64, 13_f64);
    let mut hist = Histogram::new();
    hist.fill(&gen, 100);
    let p_leng = hist.initialize();
    println!("Period: {}", p_leng);
    let max = hist.max();
    let min = hist.min(max);
    hist.count_vals_occurance(max, min);
    println!("Evenly: {}", hist.evenly_check());

    let math = hist.math();
    let disp = hist.disp(math);
    let aver_disp = Histogram::msquare(disp);

    hist.retrive_occurance(print);

    println!("{} {} {}", math, disp, aver_disp);
}
