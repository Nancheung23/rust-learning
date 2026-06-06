pub fn ifelse_example(condition: bool) {
    let number = if condition { 5 } else { 6 };
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is divisible by 6");
    }
}

pub fn forloop_example() {
    for i in 1..=5 {
        println!("\t{}", i);
    }
}

pub fn forloop_mutable_example() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    for i in &mut array {
        // change every element
        *i += 1;
        print!("{}\t", i);
        // calculate sum after change
    }
    println!();
    // manually add sum type
    let sum: i32 = array.iter().sum();
    println!("sum: {}", sum)
}

pub fn count_100() {
    for i in 1..=100 {
        if i % 2 != 0 {
            continue;
        } else if i % 45 == 0 {
            break;
        }
        print!("{} ", i);
    }
    println!();
}

pub fn enumerate_example() {
    let a = [4, 3, 2, 1];

    // 通过索引和值的方式迭代数组 `a`
    for (i, v) in a.iter().enumerate() {
        println!("No.{}'s element is {}", i + 1, v);
    }
}
