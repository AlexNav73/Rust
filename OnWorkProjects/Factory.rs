
#[derive(Debug)]
enum Test {
    One { a: i32, b: u32 },
    Two(i32, i32),
    Three
}

trait Creator {
    fn create(&self) -> Test;
}

struct A;

impl Creator for A {
    fn create(&self) -> Test {
        Test::One { a: 5, b: 13 }
    }
}

struct B;

impl Creator for B {
    fn create(&self) -> Test {
        Test::Two(15, 17)
    }
}

struct C<'a, T: 'a> {
    _m: std::marker::PhantomData<&'a T>
}

impl<'a, T: 'a> C<'a, T> {
    fn new() -> C<'a, T> {
        C { _m: std::marker::PhantomData }
    }
}

impl<'a> Creator for C<'a, i32> {
    fn create(&self) -> Test {
        Test::Three
    }
}

struct Factory<'a> {
    creators: Vec<Box<Creator + 'a>>
}

impl<'a> Factory<'a> {
    fn new() -> Factory<'a> {
        Factory { creators: Vec::new() }
    }
    
    fn reg_creator<T: Creator + 'a>(&mut self, creator: T) -> &mut Self {
        self.creators.push(Box::new(creator));
        self
    }
    
    fn resolve<'b>(&'b self, i: usize) -> &'b Creator {
        &*self.creators[i]
    }
}

fn main() {

    let mut factory = Factory::new();
    factory.reg_creator(A)
           .reg_creator(B)
           .reg_creator(C::new());

    let obj = factory.resolve(1).create();
    println!("{:?}", obj);

}

