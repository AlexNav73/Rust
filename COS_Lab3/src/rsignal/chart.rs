
use ::signal::*;
use ::rand::Rng;

pub struct Chart {
    test_signal: [f32; N],
    poly_signal: [f32; N],
    sins: [f32; N],
    rand: ::rand::ThreadRng
}

impl Chart {

    pub fn new() -> Chart {
        let mut chart = Chart {
            test_signal: [0.0; N],
            poly_signal: [0.0; N],
            sins: [0.0; N],
            rand: ::rand::thread_rng()
        };

        chart.cache_sins();
        chart.emit_test_signal();
        chart.emit_poly_signal();

        chart
    }

    fn cache_sins(&mut self) {
        for i in (0..N) {
            self.sins[i] = (2.0 * PI * i as f32 / N as f32).sin();
        }
    }

    fn emit_test_signal(&mut self) {
        for i in 0..N {
            self.test_signal[i] = signal(10.0, 0.0, i as f32, 1.0);
        }
    }

    fn emit_poly_signal(&mut self) {
        for i in 0..N {
            let mut sum = 0.0f32;
            for j in 1..30 {
                sum += poly_signal(
                    self.rand.gen::<usize>() % 7, 
                    self.rand.gen::<usize>() % 6, 
                    i as f32, 
                    j as f32);
            }
            self.poly_signal[i] = sum;
        }
    }

    pub fn show_test_signal<C: Fn(f32)>(&self, callback: C) {
        for y in self.test_signal.iter() {
            callback(*y);
        }
    }

    pub fn show_poly_signal<C: Fn(f32)>(&self, callback: C) {
        for y in self.poly_signal.iter() {
            callback(*y);
        }
    }

    pub fn recover_signal<C: Fn(f32)>(&self, sig: &[f32], add: bool, use_fi: bool, callback: C) {
        let mut amps = [0.0f32; N / 2];
        let mut fase = [0.0f32; N / 2];

        for i in 0..(N / 2) {
            fase[i] = fi(sig, &self.sins, i);
            amps[i] = aj(sig, &self.sins, i);
        }

        for i in 0..N {
            let mut sum = if add { amps[0] / 2.0 } else { 0.0f32 };
            for j in 1..(N / 2) {
                let fi = if use_fi { fase[j] } else { 0.0 };
                sum += signal(amps[j], fi, i as f32, j as f32);
            }
            callback(sum);
        }
    }

    pub fn show_recovered_signal<C: Fn(f32)>(&self, callback: C) {
        self.recover_signal(&self.test_signal, false, true, callback);
    }

    pub fn show_recovered_poly_signal<C: Fn(f32)>(&self, callback: C) {
        self.recover_signal(&self.poly_signal, true, true, callback);
    }

    pub fn show_recovered_poly_signal_without_fi<C: Fn(f32)>(&self, callback: C) {
        self.recover_signal(&self.poly_signal, true, false, callback);
    }

}

impl Drop for Chart {
    fn drop(&mut self) {
        println!("Chart has been dropped!");
    }
}
