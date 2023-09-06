use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let mut a: i32 = 5;

    let b: &mut i32 = &mut 6;

    *b += 1;
    let mut c = a + *b;

    println!("a : {}, b: {}, c: {}", a, b, c);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
