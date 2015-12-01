
use ::signal::*;
use ::rand::Rng;

//pub const APS: [u8; 7] = [ 1, 3, 4, 10, 11, 14, 17];
const APS: [u8; 7] = [ 1, 3, 5, 8, 10, 12, 16];
const FIS: [f64; 6] = [PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, 3.0 * PI / 4.0, PI];

pub struct Chart {
    test_signal: [f64; N],
    poly_signal: [f64; N],
    sins: [f64; N],
    without_fi: bool,
    rand: ::rand::ThreadRng
}

impl Chart {

    pub fn new(without_fi: bool) -> Chart {
        let mut chart = Chart {
            test_signal: [0.0; N],
            poly_signal: [0.0; N],
            sins: [0.0; N],
            without_fi: without_fi,
            rand: ::rand::thread_rng()
        };

        chart.cache_sins();
        chart.emit_test_signal();
        chart.emit_poly_signal();

        chart
    }

    fn cache_sins(&mut self) {
        for i in (0..N) {
            self.sins[i] = (2.0 * PI * i as f64 / N as f64).sin();
        }
    }

    fn emit_test_signal(&mut self) {
        for i in 0..N {
            //self.test_signal[i] = signal(20.0, 0.0, i as f64, 10.0);
            self.test_signal[i] = signal(10.0, 0.0, i as f64, 1.0);
        }
    }

    fn emit_poly_signal(&mut self) {
        for i in 0..N {
            let mut sum = 0.0f64;
            for j in 1..30 {
                let fi = if self.without_fi { 0.0 } else { FIS[self.rand.gen::<usize>() % 6] as f64 };
                sum += signal(
                    APS[self.rand.gen::<usize>() % 7] as f64,
                    fi,
                    i as f64,
                    j as f64);
            }
            self.poly_signal[i] = sum;
        }
    }

    fn show_signal<C: Fn(f64)>(sig: &[f64], callback: C) {
        for y in sig.iter() {
            callback(*y);
        }
    }

    pub fn specter<C, F>(&self, sig: &[f64], predicate: F, callback: C)
        where C: Fn(f64),
              F: Fn(&[f64], &[f64], usize) -> f64 {
        for j in 0..N {
            callback(predicate(sig, &self.sins, j));
        }
    }

    pub fn recover_signal<C: Fn(f64)>(&self, sig: &[f64], add: bool, callback: C) {
        let mut amps = [0.0f64; N / 2];
        let mut fase = [0.0f64; N / 2];

        for i in 0..(N / 2) {
            amps[i] = aj(sig, &self.sins, i);
            fase[i] = fi(sig, &self.sins, i);
        }

        for i in 0..N {
            let mut sum = if add { amps[0] / 2.0 } else { 0.0f64 };
            for j in 1..(N / 2) {
                let fi = if !self.without_fi { fase[j] } else { 0.0 };
                sum += signal(amps[j], fi, i as f64, j as f64);
            }
            callback(sum);
        }
    }

    pub fn show_test_signal<C: Fn(f64)>(&self, callback: C) {
        Self::show_signal(&self.test_signal, callback);
    }

    pub fn show_poly_signal<C: Fn(f64)>(&self, callback: C) {
        Self::show_signal(&self.poly_signal, callback);
    }

    pub fn show_test_aj_specter<C: Fn(f64)>(&self, callback: C) {
        self.specter(&self.test_signal, aj, callback);
    }

    pub fn show_test_fi_specter<C: Fn(f64)>(&self, callback: C) {
        self.specter(&self.test_signal, fi, callback);
    }

    pub fn show_poly_aj_specter<C: Fn(f64)>(&self, callback: C) {
        self.specter(&self.poly_signal, aj, callback);
    }

    pub fn show_poly_fi_specter<C: Fn(f64)>(&self, callback: C) {
        self.specter(&self.poly_signal, fi, callback);
    }
    pub fn show_recovered_signal<C: Fn(f64)>(&self, callback: C) {
        self.recover_signal(&self.test_signal, false, callback);
    }

    pub fn show_recovered_poly_signal<C: Fn(f64)>(&self, callback: C) {
        self.recover_signal(&self.poly_signal, true, callback);
    }

    pub fn show_recovered_poly_signal_without_fi<C: Fn(f64)>(&self, callback: C) {
        self.recover_signal(&self.poly_signal, true, callback);
    }

}

impl Drop for Chart {
    fn drop(&mut self) {
        println!("Chart has been dropped!");
    }
}
