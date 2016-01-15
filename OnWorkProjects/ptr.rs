
macro_rules! shift_ptr {
    ($lhs:expr, [$($t:ty), *]) => {
        {
            let ptr = &mut $lhs as *mut _;
            $(
                let ptr = unsafe { (ptr as *mut $t).offset(1) };
            )*
            ptr as *mut _
        }
    }
}

#[allow(dead_code)]
struct Test {
    a: i32,
    b: i32,
    c: u64
}

#[allow(dead_code)]
struct UsizeTest {
    a: i32,
    b: usize,
    c: u64
}

#[allow(dead_code)]
struct IsizeTest {
    a: i32,
    b: isize,
    c: u64
}

#[allow(dead_code)]
struct CharTest {
    a: i32,
    b: char,
    c: u64
}

#[allow(dead_code)]
struct StringTest {
    a: String,
    b: String
}

#[allow(dead_code)]
fn test_isize_shifting() {
    let mut t = UsizeTest { a: 5, b: 11, c: 256 };
    
    let a_ptr: *mut i32 = shift_ptr!(t.a, []);
    let b_ptr: *mut isize = shift_ptr!(t.a, [isize]);
    let c_ptr: *mut u64 = shift_ptr!(t.a, [isize, isize]);
    
    unsafe { println!("{}", *a_ptr); }
    unsafe { println!("{}", *b_ptr); }
    unsafe { println!("{}", *c_ptr); }
}

#[allow(dead_code)]
fn test_usize_shifting() {
    let mut t = UsizeTest { a: 5, b: 11, c: 256 };
    
    let a_ptr: *mut i32 = shift_ptr!(t.a, []);
    let b_ptr: *mut usize = shift_ptr!(t.a, [usize]);
    let c_ptr: *mut u64 = shift_ptr!(t.a, [usize, usize]);
    
    unsafe { println!("{}", *a_ptr); }
    unsafe { println!("{}", *b_ptr); }
    unsafe { println!("{}", *c_ptr); }
}

#[allow(dead_code)]
fn test_i32_shifting() {
    let mut t = Test { a: 5, b: 11, c: 256 };
    
    let a_ptr: *mut i32 = shift_ptr!(t.a, []);
    let b_ptr: *mut i32 = shift_ptr!(t.a, [i32]);
    let c_ptr: *mut u64 = shift_ptr!(t.a, [i32, i32]);
    
    unsafe { println!("{}", *a_ptr); }
    unsafe { println!("{}", *b_ptr); }
    unsafe { println!("{}", *c_ptr); }
}

#[allow(dead_code)]
fn test_char_shifting() {
    let mut t = CharTest { a: 5, b: 'a', c: 256 };
    
    let a_ptr: *mut i32 = shift_ptr!(t.a, []);
    let b_ptr: *mut char = shift_ptr!(t.a, [i32]);
    let c_ptr: *mut u64 = shift_ptr!(t.a, [i32, char]);
    
    unsafe { println!("{}", *a_ptr); }
    unsafe { println!("{}", *b_ptr); }
    unsafe { println!("{}", *c_ptr); }
}

fn test_string_shifting() {

    let mut t = StringTest { a: "Hello".to_string(), b: " World".to_string() };

    let string = {
        let len = t.a.len();
        let r: &mut str = unsafe { t.a.slice_mut_unchecked(0, len) };
        let p = r.as_ptr() as *mut u8;
        
        let p: *mut u8 = unsafe { p.offset(len as isize) };
        
        let len = len + t.b.len();
        unsafe { String::from_raw_parts(p, len, len + 1) }
    };
    
    println!("{}{}", t.a, string);
}

fn main() {
    //test_isize_shifting();
    //test_usize_shifting();
    //test_i32_shifting();
    //test_char_shifting();
    //test_string_shifting();
    test_string_shifting();
}
