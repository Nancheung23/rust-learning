// greeting
fn greet_world() {
    let chinese = "世界，你好！";
    let english = "world, hello!";
    let finnish = "maailmaa, terve!";
    let regions = [chinese, english, finnish];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
// sum
fn add(i:i32,j:i32) -> i32 {
    return i + j;
}

fn main() {
    greet_world();
    // const a, b
    let a = 10;
    let b: i32 = 20;
    // mutable c
    let mut c = 30i32;
    // _ for better reading xp
    let d = 30_i32;
    // use function return a parameter
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}
