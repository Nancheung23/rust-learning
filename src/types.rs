pub fn init() {
    println!("{}", file!());
}

use num::complex::Complex;
// space count
pub fn space_count() {
    let space = "      ";
    let space = space.len();
    println!("space count: {}", space);
}
// numeric type: i8->i128, isize. u8->u128, usize. f32/ f64. default in rust: i32
// string type: &str ; char type: char for one unicode character such as 'A'
// boolean: true/ false
// unit type: (), same as the only value
pub fn guess_type() {
    let msg: &str = "Not a number!";
    let guess: i32 = "42".parse().expect(msg);
    println!("the number is: {}", guess);
    // float
    let mut x = 2.0;
    let y: f32 = 3.1;
    println!("x + y = {}", x + y);
    // NAN
    x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("sqrt result: {}", x);
    }
}
// Range
pub fn range_usage() {
    let numbers = [1, 2];
    for i in 1..=3 {
        for num in numbers.iter() {
            println!("{} : {} ", i, num);
        }
    }
    for i in 'a'..='c' {
        print!("{} ", i);
    }
    println!();
}
// num 0.4.0 dependency
pub fn num_usage() {
    // to represent: k + ji
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im)
}
// char_val
pub fn char_val() {
    let x = 'a';
    println!("letter 'a' occupies {} bytes", size_of_val(&x));
}
