#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[derive(Debug)]
pub enum Direction {
    East,
    West,
    South,
    North,
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn direction_select(direction: Direction) {
    match direction {
        Direction::East => println!("The direction is East"),
        Direction::North | Direction::South => {
            println!("The direction is either North or South");
        }
        Direction::West => println!("The direction is West!!"),
    };
}

pub fn match_div_value(value: u8) {
    match value {
        2 => println!("two"),
        4 => println!("four"),
        6 => println!("six"),
        8 => println!("eight"),
        // can use () unit as return, means literally do nothing, also can use a variable like other => to demonstrate _ if prefer
        _ => println!("other value"),
    };
}

pub fn iflet_div_value(value: Option<u8>) {
    if let Some(8) = value {
        println!("eight");
    }
}

pub fn whilelet_example(range: u8) {
    let mut stack = Vec::new();
    for i in 1..=range {
        stack.push(i);
    }
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn letelse_example(index: Option<u8>) {
    // if let Some(index) = index {
    //     let result = index * 10;
    //     println!("result after 10x: {:?}", result);
    // } else {
    //     println!("end")
    // }
    let Some(index) = index else {
        println!("end");
        return;
    };
    let result = index * 10;
    println!("result after 10x: {:?}", result);
    println!("init value is:{:?}", index);
}
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn ignore_patterns(point: Point) {
    match point {
        Point {
            x: 255,
            y: 255,
            z: 255,
        } => println!("origin dot"),
        Point { y: 0, z: 0, .. } => println!("x axis"),
        Point { x: 0, z: 0, .. } => println!("y axis"),
        Point { x: 0, y: 0, .. } => println!("z axis"),
        _ => println!("in scope"),
    };
}

pub enum Weather {
    Sunny(i8),
    Rainy(i8),
    Snowy(i8),
    Cloudy(i8),
    Windy(i8),
}

pub fn travel(weather: Weather) -> bool {
    match weather {
        Weather::Sunny(temp) if temp < 0 => false,
        Weather::Sunny(_temp) => true,
        Weather::Rainy(temp) if temp < 15 => false,
        Weather::Rainy(_temp) => true,
        Weather::Snowy(temp) if temp < -5 => false,
        Weather::Snowy(_temp) => true,
        Weather::Cloudy(temp) if temp < 10 => false,
        Weather::Cloudy(_temp) => true,
        Weather::Windy(temp) if temp < 20 => false,
        Weather::Windy(_temp) => true,
    }
}

pub fn variable_bind(value: u8) {
    match value {
        val @ 50..100 => {
            println!("high value: {:?}", val);
        }
        0..50 => {
            println!("low value: {:?}", value);
        }
        _ => {
            println!("exceed limit");
        }
    }
}
