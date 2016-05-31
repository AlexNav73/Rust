
const N: f32 = 512_f32;

pub struct Signal {
    a: f32,
    f: f32,
    fi: f32,
}

impl Signal {

    pub fn new(a: f32, f: f32, fi: f32) -> Signal {
        Signal { a: a, f: f, fi: fi }
    }

    fn step(n: f32, a: f32, f: f32, fi: f32) -> f32 {
        a * ((2_f32 * ::std::f32::consts::PI * f * n / N) + fi).cos()
    }

    pub fn graph1<C>(&self, idx: u32, j: f32, callback: C) where C : Fn(u32, f32, f32) {
        if idx == 999 {  panic!("Values slice is empty") }

        let fi = 7.0 + j as f32 / 6.0;
        for i in (0_u32..N as u32) {
            callback(
                idx, i as f32,
                self.a * (2.0 * ::std::f32::consts::PI * self.f * i as f32 / N + fi)
            )
        }
    }

    pub fn graph2<C>(&self, idx: u32, j: f32, callback: C) where C : Fn(u32, f32, f32) {
        if idx == 999 {  panic!("Values slice is empty") }

        let fi = 7.0 + j as f32 / 6.0;
        for i in (0_u32..N as u32) {
            callback(
                idx, i as f32, 
                Self::step(i as f32, self.a, self.f, fi)
            )
        }
    }

    pub fn graph3<C>(&self, callback: C) where C : Fn(u32, f32, f32) {
        for i in (0_u32..N as u32) {
            let mut sum = 0_f32;
            for j in (1_u32..5+1) {
                let ai = self.a + j as f32;
                let fith = self.f + j as f32;
                let fi = 7.0 + j as f32 / 6.0;
                sum += Self::step(i as f32, ai, fith, fi)
            }
            callback(0, i as f32, sum);
        }
    }

}

impl Drop for Signal {
    fn drop(&mut self) {
        println!("Signal has been dropped!");
    }
}
