
use super::traits::*;
use super::rand::{ Rng, ThreadRng };

pub struct IterGen<'a> {
    gen: &'a Generator,
    sequance: Vec<f64>,
    count: u32,
    random: ThreadRng
}

pub struct Generator {
    expc: f64,
    mdev: f64,
    lemer_gen: super::lemer::Generator
}

impl<'a> IntoGenerator for &'a Generator {
    type Output = f64;
    type IntoGen = IterGen<'a>;

    fn into_gen(self, count: u32) -> Self::IntoGen {
        let mut iter = IterGen { 
            gen: self, 
            sequance: Vec::new(), 
            count: 0_u32,
            random: super::rand::thread_rng()
        };
        let mut it = self.lemer_gen.into_gen(count);
        while let Some(next) = it.generate() {
            iter.sequance.push(next);
        }
        iter.count = iter.sequance.len() as u32;
        iter
    }
}

impl<'a> Generate for IterGen<'a> {
    type Output = f64;

    fn generate(&mut self) -> Option<Self::Output> {
        if self.count > 0 {
            let mut sum = 0_f64;
            for _i in (0..6) {
                sum += self.sequance[
                    self.random.gen::<usize>() % (self.sequance.len() - 1)
                ];
            }
            self.count -= 1;
            Some(self.gen.expc + self.gen.mdev * 2_f64.sqrt() * (sum - 3_f64))
        } else { None }
    }
}

impl Generator {
    pub fn new(a: f64, seed: f64, m: f64, expc: f64, mdev: f64) -> Generator {
        Generator {
            expc: expc,
            mdev: mdev,
            lemer_gen: super::lemer::Generator::new(a, seed, m),
        }
    }
}
