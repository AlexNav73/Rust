
use super::traits::*;

pub struct IterGen<'a> {
    gen: &'a Generator,
    lemer: super::lemer::IterGen<'a>
}

pub struct Generator {
    l: f64,
    r: f64,
    lemer_gen: super::lemer::Generator
}

impl<'a> IntoGenerator for &'a Generator {
    type Output = f64;
    type IntoGen = IterGen<'a>;

    fn into_gen(self, count: u32) -> Self::IntoGen {
        IterGen { gen: self, lemer: self.lemer_gen.into_gen(count) }
    }
}

impl<'a> Generate for IterGen<'a> {
    type Output = f64;

    fn generate(&mut self) -> Option<Self::Output> {
        if let Some(next) = self.lemer.generate() {
            Some(self.gen.l + (self.gen.r - self.gen.l) * next)
        } else { None }
    }
}

impl Generator {
    pub fn new(a: f64, seed: f64, m: f64, l: f64, r: f64) -> Generator {
        Generator {
            l: l,
            r: r,
            lemer_gen: super::lemer::Generator::new(a, seed, m),
        }
    }
}
