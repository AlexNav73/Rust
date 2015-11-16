
use ::signal::*;
use ::rand::Rng;

pub struct Chart {
    sins: [f32; N as usize],
    rand: ::rand::ThreadRng
}

impl Chart {

    pub fn new() -> Chart {
        let mut chart = Chart {
            sins: [0.0; N as usize],
            rand: ::rand::thread_rng()
        };
        chart.cache_sins();
        chart
    }

    fn cache_sins(&mut self) {
        for i in (0..N) {
            self.sins[i as usize] = (2.0 * PI * i as f32 / N as f32).sin();
        }
    }

    pub fn show_poly_signal<C: Fn(f32)>(&mut self, callback: C) {
        for i in (0..N) {
            let mut sum = 0.0f32;
            for _j in 1..30 {
                sum += poly_signal(self.rand.gen::<usize>() % 7, self.rand.gen::<usize>() % 6, i as f32, Option::None);
            }
            callback(sum);
        }
    }

    pub fn show_spectors<C: Fn(f32)>(&mut self, callback: C) {
        let mut amps = [0.0f32; (N / 2) as usize];
        let mut fase = [0.0f32; (N / 2) as usize];

        let mut seq = [0.0f32; N as usize];
        for i in (0..N) {
            let mut sum = 0.0f32;
            for _j in 1..30 {
                sum += poly_signal(self.rand.gen::<usize>() % 7, self.rand.gen::<usize>() % 6, i as f32, Option::None);
            }
            seq[i as usize] = sum;
        }

        for j in 1..(N / 2) {
            println!("1");
            fase[j as usize] = fi(&seq, &self.sins, j as usize);
            println!("2");
            amps[j as usize] = aj(&seq, &self.sins, j as usize);
            println!("3");
        }

        for i in 0..N {
            let mut sum = 0.0f32;
            for j in 1..(N / 2) {
                sum += signal(amps[j as usize], 0.0, i as f32, Option::Some(j as f32));
            }
            callback(sum);
        }
    }

}

impl Drop for Chart {
    fn drop(&mut self) {
        println!("Chart has been dropped!");
    }
}
