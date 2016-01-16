use std::ops::Index;

struct Test {
    inner: Box<i32>
}

impl Index<usize> for Test {
    type Output = i32;

    #[inline]
    fn index(&self, index: usize) -> &i32 {
        print!("index: {} = ", index);
        &*self.inner
    }
}

fn main() {
    let t = Test { inner: Box::new(9) };
    
    for i in 0..5 {
        println!("{}", t[i]);
    }
}
