// add function for expression
pub fn add_num(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    // return expression without keyword
    x + y
}
// expression
pub fn give_value(x: i32) -> i32 {
    let y = { x + 1 };
    y
}
