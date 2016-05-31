
use gens::traits::*;

const INTERVALS: u32 = 20;

pub struct Histogram {
    sequance: Vec<f64>,
    period: Vec<f64>,
    occurance: [f64; INTERVALS as usize]
}

impl Histogram {

    pub fn new() -> Histogram {
        Histogram {
            sequance: Vec::new(),
            period: Vec::new(),
            occurance: [0.0; INTERVALS as usize]
        }
    }

    pub fn fill<'a, G>(&mut self, gen: &'a G, count: u32)
        where &'a G: IntoGenerator<Output=f64> {

        self.sequance.clear();
        self.period.clear();
        for x in &mut self.occurance {
            *x = 0_f64;
        }

        let mut iter = gen.into_gen(count);
        while let Some(next) = iter.generate() {
            self.sequance.push(next);
        }
    }

    pub fn max(&self) -> f64 {
        self.period.iter().fold(0.0, |acc, &x| if x > acc { x } else { acc })
    }

    pub fn min(&self, max: f64) -> f64 {
        self.period.iter().fold(max, |acc, &x| if x < acc { x } else { acc })
    }

    pub fn initialize(&mut self) -> usize {
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        {
            let last = match self.sequance.last() {
                Some(l) => l,
                None => return 0
            };

            for (index, f) in self.sequance.iter().enumerate() {
                if i1 == 0 && *last == *f {
                    i1 = index;
                } else if i2 == 0 && *last == *f {
                    i2 = index;
                    break;
                }
            }
        }

        if i1 < i2 {
            for x in (i1..i2) { self.period.push(self.sequance[x]); }
        } else {
            for x in (i2..i1) { self.period.push(self.sequance[x]); }
        }
        self.period.len()
    }

    fn occurance_to_ordinats(&mut self) {
        let period_len = self.period.len();
        for x in &mut self.occurance {
            if *x != 0_f64 { *x /= period_len as f64; }
        }
    }

    pub fn count_vals_occurance(&mut self, max: f64, min: f64) {
        let delta = (max - min) / INTERVALS as f64;

        for x in &self.period {
            let index = ((*x - min) / delta).floor() as usize;
            let len = INTERVALS as usize;
            if index < len {
                self.occurance[index] += 1.0;
            } else {
                self.occurance[len - 1] += 1.0;
            }
        }
        self.occurance_to_ordinats();
    }

    pub fn evenly_check(&self) -> f64 {
        let mut total_pairs: f64 = 0.0;
        let mut evenly_pairs: f64 = 0.0;
        let mut index: usize = 0;
        let p_len = self.sequance.len() / 2_usize;

        while index < p_len {
            if (self.sequance[index].powi(2) + self.sequance[index].powi(2)) < 1.0_f64 {
                evenly_pairs += 1.0;
            }
            total_pairs += 1.0;
            index += 2;
        }
        evenly_pairs / total_pairs
    }

    pub fn math(&self) -> f64 {
        let sum = self.sequance.iter().fold(0.0_f64, |acc, &x| acc + x);
        sum / self.sequance.len() as f64
    }

    pub fn disp(&self, math: f64) -> f64 {
        let sum = self.sequance.iter().fold(0.0_f64, |acc, &x| acc + (x - math).powi(2));
        sum / self.sequance.len() as f64
    }

    pub fn msquare(disp: f64) -> f64 {
        disp.sqrt()
    }

    pub fn retrive_occurance<C>(&self, callback: C) where C: Fn(f64) {
        for x in &self.occurance {
            callback(*x);
        }
    }

    pub fn retrive_sequance<C>(&self, callback: C) where C: Fn(f64) {
        for x in &self.sequance {
            callback(*x);
        }
    }

}
