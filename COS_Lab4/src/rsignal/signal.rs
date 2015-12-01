
use ::rand::Rng;

const N: usize = 512;
const PI: f64 = ::std::f64::consts::PI;

pub struct Signal {
    pub xs: [f64; N]
}

impl Signal {

    pub fn new(b1: f64, b2: f64) -> Signal {
        let mut s = Signal {
            xs: [0.0; N]
        };

        let mut rnd = ::rand::thread_rng();
        for i in 0..N {
            let mut sum = 0.0f64;
            for j in 50..70 {
                let sign = if rnd.gen() { -1.0 } else { 1.0 };
                sum += sign * b2 * (2.0 * PI * i as f64 * j as f64 / N as f64).sin();
            }
            s.xs[i] = sum + b1 * (2.0 * PI * i as f64 / N as f64).sin();
        }
        s
    }

    pub fn show_signal<C: Fn(f64)>(&self, callback: C) {
        for y in self.xs.iter() {
            callback(*y);
        }
    }

}

impl Drop for Signal {
    fn drop(&mut self) {
        println!("Signal has been dropped!");
    }
}
