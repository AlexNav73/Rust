
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

#[allow(dead_code)]
fn test_string_shifting() {

    let mut t = StringTest { a: "Hello".to_string(), b: "_World".to_string() };

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

use std::collections::HashMap;

macro_rules! Struct {
    ($name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug)]
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            fn new(data: HashMap<&str, &str>) -> $name {
                $name {
                    $(
                    $field_name: data[stringify!($field_name)].parse::<$field_type>().unwrap(),
                    )*
                }
            }
        }
    }
}

Struct! { 
    Tmp {
        a: i32,
        b: u64,
    }
}

fn main() {

    let mut h = HashMap::new();
    h.insert("a", "10");
    h.insert("b", "30");
    let t = Tmp::new(h);
    println!("{:?}", t);

    //test_isize_shifting();
    //test_usize_shifting();
    //test_i32_shifting();
    //test_char_shifting();
    //test_string_shifting();
    
    //let mut t = StringTest { a: "Hello".to_string(), b: "_World".to_string() };
    //
    //let mut a_ptr: *mut String = &mut t.a as *mut String;
    //a_ptr = unsafe { a_ptr.offset(24 as isize) };
    //let us: *mut [usize; 3] = unsafe { std::mem::transmute(a_ptr) };
    //let ref_us: &[usize; 3] = unsafe { std::mem::transmute(us) };
    //
    //println!("capacity: {}", ref_us[2]);
    //println!("len: {}", ref_us[1]);
    //println!("ptr: {}", ref_us[0]);
    //
    //println!("\"{}\"", unsafe { String::from_raw_parts(ref_us[0] as *mut u8, ref_us[1], ref_us[2]) });
    
}
