 
/// 
/// Evaluete sum of even numbers in Fibonacci sequince.
///
/// Terms in sequince needs to be less then 4 million.
///

fn main() {

    let mut vec: Vec<u32> = vec![1, 2];
    let mut i = 0;

    //
    // Start from 2, because check to eveness starts from 3-rd 
    // term, and second ( == 2 ) forgets.
    //

    let mut sum: u32 = 2;

    loop {

        let res: u32;
        {
            let a: &u32 = &vec[i - 2];
            let b: &u32 = &vec[i - 1];
            res = *a + *b;
        }
        if res >= 4000000 { break; }
        if res % 2 == 0 { sum += res; }
        vec.push(res);
        i += 1;

    }

    println!("{:?}", vec);
    println!("Sum: {:?}", sum);
}

