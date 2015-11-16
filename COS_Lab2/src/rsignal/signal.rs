
const MAGIC: u32 = 5;
pub const N: u32 = 512;

pub struct Signal {
    a: f32,
    f: f32,

    pub sequance: [f32; (MAGIC * N) as usize]
}

impl Signal {
    pub fn new(a: f32, f: f32) -> Signal {
        Signal { a: a, f: f, sequance: [0.0; (MAGIC * N) as usize] }
    }

    fn step(n: f32, a: f32, f: f32, fi: f32) -> f32 {
        a * ((2_f32 * ::std::f32::consts::PI * f * n / N as f32) + fi).cos()
    }

    pub fn source_graph(&mut self, _j: f32) {
        for i in (0..MAGIC * N) {
            let mut sum = 0.0;
            for j in (1_u32..5+1) {
                let ai = self.a + j as f32;
                let fith = self.f + j as f32;
                let fi = 7.0 + j as f32 / 6.0;
                sum += Self::step(i as f32, ai, fith, fi)
            }
            self.sequance[i as usize] = sum;
        }
    }

    pub fn graph0<C: Fn(f32, f32)>(&mut self, callback: C) {
        self.source_graph(1.0);
        for i in (0..MAGIC * N) {
            callback(i as f32, self.sequance[i as usize]);
        }
    }

    pub fn graph1<C: Fn(f32, f32)>(&mut self, callback: C) {
        self.source_graph(1.0);
        let math = self.math();
        let mut y = 0.0;
        for i in (0..MAGIC * N) {
            y = (self.sequance[i as usize] - math).powi(2);
            self.sequance[i as usize] = y;
            callback(i as f32, y);
        }
    }

    pub fn graph2<C: Fn(f32, f32)>(&mut self, callback: C) {
        for k in (0..MAGIC * N) {
            let mut s = 0.0;
            for n in (0..MAGIC * N) {
                let mut sum = 0.0;
                for j in (1_u32..5+1) {
                    let ai = self.a + j as f32;
                    let fith = self.f + j as f32;
                    let fi = 7.0 + j as f32 / 6.0;
                    sum += Self::step(n as f32 + k as f32, ai, fith, fi) * 
                           Self::step(n as f32, ai, fith, fi)
                }
                s += sum;
                self.sequance[k as usize] = s / N as f32;
            }
            callback(k as f32, s / N as f32);
        }
    }

    pub fn math(&self) -> f32 {
        let sum: f32 = self.sequance.iter().fold(0.0, |acc, &x| acc + x);
        sum / (MAGIC * N) as f32
    }

    pub fn disp(&self) -> f32 {
        let math = self.math();
        let sum: f32 = 
            self.sequance.iter().fold(0.0, |acc, &x| acc + (x - math).powi(2));
        sum / (MAGIC * N) as f32
    }

}

impl Drop for Signal {
    fn drop(&mut self) {
        println!("Object has been dropped!");
    }
}
