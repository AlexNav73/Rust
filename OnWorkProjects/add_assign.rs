
use std::ops::Add;

trait AddAssign<RHS: Add<Output = Self>> where Self: Sized + Add<RHS> {
    fn add_assign(self, rhs: RHS) -> Self::Output {
        self + rhs
    }
}

#[derive(Copy, Clone)]
struct Test {
    a: i32,
    b: u32
}

impl Add for Test {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Test { a: self.a + rhs.a, b: self.b }
    }
}

impl AddAssign<Test> for Test {
}

fn main() {

    let t1 = Test { a: 1, b: 3 };
    let t2 = Test { a: 2, b: 3 };
    
    println!("{}", (t1 + t2).a);
    println!("{}", (t1.add_assign(t2)).a);
}

