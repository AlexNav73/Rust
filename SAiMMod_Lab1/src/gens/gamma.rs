
use super::traits::*;
use super::rand::{ Rng, ThreadRng };

pub struct IterGen<'a> {
    gen: &'a Generator,
    sequance: Vec<f64>,
    count: u32,
    random: ThreadRng
}

pub struct Generator {
    eta: f64,
    lambda: f64,
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
            self.count -= 1;
            let mut r = 1_f64;
            for _x in (0..self.gen.eta.floor() as u32) {
                r *= self.sequance[
                    self.random.gen::<usize>() % (self.sequance.len() - 1)
                ]
            }
            Some(-(r.ln() / self.gen.lambda))
        } else { None }
    }
}

impl Generator {
    pub fn new(a: f64, seed: f64, m: f64, aa: f64, b: f64) -> Generator {
        Generator {
            eta: aa,
            lambda: b,
            lemer_gen: super::lemer::Generator::new(a, seed, m),
        }
    }
}
