
use super::traits::*;

pub struct IterGen<'a> {
    count: u32,
    seed: f64,
    gen: &'a Generator
}

pub struct Generator {
    a: f64,
    seed: f64,
    m: f64
}

impl<'a> IntoGenerator for &'a Generator {
    type Output = f64;
    type IntoGen = IterGen<'a>;

    fn into_gen(self, count: u32) -> Self::IntoGen {
        IterGen { count: count, seed: self.seed, gen: self }
    }
}

impl<'a> Generate for IterGen<'a> {
    type Output = f64;

    fn generate(&mut self) -> Option<Self::Output> {
        if self.count > 0 {
            let new_seed = (self.gen.a * self.seed) % self.gen.m;
            self.seed = new_seed;
            self.count -= 1;
            Some(new_seed / self.gen.m)
        } else { None }
    }
}

impl Generator {
    pub fn new(a: f64, seed: f64, m: f64) -> Generator {
        Generator { a: a, seed: seed, m: m }
    }
}
