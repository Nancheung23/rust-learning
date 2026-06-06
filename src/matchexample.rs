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
