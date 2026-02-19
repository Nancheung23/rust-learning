use num::complex::Complex;

// a strct for number 
struct Struct {
    e: i32
}
// const
const MAX_POINTS: u32 = 100_000;

// greeting
fn greet_world() {
    println!("{:?}, world!", define_x());
    let chinese = "世界，你好！";
    let english = "world, hello!";
    let finnish = "maailmaa, terve!";
    let regions = [chinese, english, finnish];
    for region in regions.iter() {
        println!("{}", &region);
    }
    println!("Max points: {}", MAX_POINTS);
}
// sum
fn add(i:i32,j:i32) -> i32 {
    i + j
}
//use case of add
fn add_usage() {
    // const a, b
    let a = 10;
    let b: i32 = 20;
    // mutable c
    let mut c = 30i32;
    // _ for better reading xp
    let d = 30_i32;
    // use function return a parameter
    let e = add(add(a, b), add(c, d));
    println!("origin: (a + b) + (c + d) = {}", e);
    // usage of mutable
    c = 50_i32;
    let f = add(add(a, b), add(c, d));
    println!("current: (a + b) + (c + d) = {}", f);
    // declare unused variable
    let _x = 5_i32;
}
// check variable
fn check_variable() {
    let mut x:i32 = 5;
    println!("The value of x is: {}", x);
    x = 6_i32;
    println!("The value of x is: {}", x);
    // _ for unused variable
    let _y = 10;
    // assign multiple variables
    let (a, mut b) : (bool, bool) = (true, false);
    println!("a={:?}, b={:?}", a, b);
    b = true;
    assert_eq!(a, b);
}
// advanced assign variables
fn adv_variable() {
    // name 5 vars from a to e
    let (a, b, c, d, e);
    // a = 1, b = 2
    (a, b) = (1, 2);
    // c, d ,e = 1, 4, 5, .. means ignore the middle numbers (rest pattern)
    [c, .. , d, _] = [1, 2, 3, 4, 5];
    // e = 5
    Struct {e, ..} = Struct { e : 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d ,e]);
}
// overwrite variable
fn overwrite_variable() {
    let x = 5;
    // become 6
    let x = x + 1;
    {
        // variable is only overwritten in the inner scope, which is 12
        let x = x * 2;
        // print 12
        println!("The value of x in the inner scope is : {}", x);
    }
    // print 6
    println!("The value of x is: {}", x);
}
// space count
fn space_count() {
    let space = "      ";
    let space = space.len();
    println!("space count: {}", space);
}
// return the static value
fn define_x() -> &'static str {
    let x = "hello";
    x
}
// return variable
fn _define_y() -> String {
    let y = String::from("hello");
    y
}
// numeric type: i8->i128, isize. u8->u128, usize. f32/ f64. default in rust: i32
// string type: &str ; char type: char for one unicode character such as 'A'
// boolean: true/ false
// unit type: (), same as the only value
fn guess_type() {
    let msg:&str = "Not a number!";
    let guess:i32 = "42".parse().expect(msg);
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
fn range_usage() {
    let numbers = [
        1, 2
    ];
    for i in 1..=3 {
        for num in numbers.iter() {
            println!("{} : {} ", i,num);
        }
    }
    for i in 'a'..='c' {
        print!("{} ",i);
    }
    println!();
}
// num 0.4.0 dependency
fn num_usage() {
    // to represent: k + ji
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im)
}

// main function
fn main() {
    greet_world();
    add_usage();
    check_variable();
    adv_variable();
    overwrite_variable();
    space_count();
    guess_type();
    range_usage();
    num_usage();
}
