pub fn init() {
    println!("{}", file!());
}
// a strct for number
pub struct Struct {
    e: i32,
}
// const
pub(crate) const MAX_POINTS: u32 = 100_000;

// return the static value
pub fn define_x() -> &'static str {
    let x = "hello";
    x
}
// return variable
pub fn _define_y() -> String {
    let y = String::from("hello");
    y
}

// greeting
pub fn greet_world() {
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
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
//use case of add
pub fn add_usage() {
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
pub fn check_variable() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6_i32;
    println!("The value of x is: {}", x);
    // _ for unused variable
    let _y = 10;
    // assign multiple variables
    let (a, mut b): (bool, bool) = (true, false);
    println!("a={:?}, b={:?}", a, b);
    b = true;
    assert_eq!(a, b);
}
// advanced assign variables
pub fn adv_variable() {
    // name 5 vars from a to e
    let (a, b, c, d, e);
    // a = 1, b = 2
    (a, b) = (1, 2);
    // c, d ,e = 1, 4, 5, .. means ignore the middle numbers (rest pattern)
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // e = 5
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
// overwrite variable
pub fn overwrite_variable() {
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
