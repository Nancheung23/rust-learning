use std::io;
pub fn array_example() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

pub fn string_array() {
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    let slice: &[String] = &array[2..=6];
    println!("{:#?}", slice);
}

pub fn two_deminsion_array() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [4, 5, 6];
    let zero = [0; 3];
    let zero1: [u8; 3] = [0; 3];
    let arrays: [[u8; 3]; 4] = [one, two, zero, zero1];
    // iterate each element in two deminsion array
    for a in &arrays {
        print!("{:?}", a);
        // short version of "for n in a.iter {}"
        for n in a {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        for i in 0..a.len() {
            // get sum of each one deminsion array
            sum += a[i];
        }
        // format print sum
        println!("\t({:?} = {})", a, sum);
    }
}
