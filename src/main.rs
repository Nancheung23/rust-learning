mod array;
mod basics;
mod enums;
mod functions;
mod ifelse;
mod ownership;
mod string;
mod types;

use basics::MAX_POINTS;

use crate::{
    array::string_array, array::two_deminsion_array, enums::print_card, ifelse::count_100,
    ifelse::enumerate_example, ifelse::forloop_example, ifelse::forloop_mutable_example,
    ifelse::ifelse_example, types::IsInvincible,
};

fn print_separator() {
    println!("{}", "=".repeat(20));
}
// main function
fn main() {
    basics::init();
    basics::greet_world();
    basics::check_variable();
    basics::adv_variable();
    basics::add_usage();
    basics::overwrite_variable();
    println!("Max points constant: {}", MAX_POINTS);
    print_separator();
    types::init();
    types::space_count();
    types::guess_type();
    types::range_usage();
    types::num_usage();
    types::char_val();
    println!("x + y = {}", functions::add_num(1, 2));
    println!("give y a value: {}", functions::give_value(3));
    print_separator();
    ownership::init();
    ownership::push_word_expamle();
    ownership::move_part();
    ownership::compare_ref(5);
    ownership::calculate_length_example();
    ownership::push_ref_example();
    print_separator();
    string::init();
    string::greet(String::from("Alice"));
    string::print_chinese();
    println!("First word: {}", string::first_word(&String::from("hello")));
    string::push_string_example();
    string::delete_string_example();
    string::concatenate_string_example();
    string::ascii_example();
    string::unicode_example();
    string::chars_example();
    print_separator();
    println!(
        "Elements in tuple example: {}, {}, {}",
        types::tuple_example().0,
        types::tuple_example().1,
        types::tuple_example().2
    );
    print_separator();
    let user = types::struct_example(
        String::from("default@example.com"),
        String::from("test user"),
    );
    println!(
        "User:\nUsername:{}\nEmail:{}\nBalance:{:?}\nTxs:{:?}\n{:?}",
        user.username, user.email, user.balance, user.txs, user.active
    );
    // pretty print
    println!("{:#?}", user);
    let user1 = types::User {
        balance: user.balance + 100.00,
        ..user
    };
    println!("updated user info by adding 100 fund {:#?}", user1);
    println!(
        "balance update:\nold: {:#?}\nnew: {:#?}",
        user.balance, user1.balance
    );
    let subject = types::AlwaysEqual;
    if subject.check_god_mode() {
        println!("trait bonded, it is INVINCIBLE!!!");
    };
    print_separator();
    let c1 = enums::PokerCard {
        suit: enums::Pokersuit::Hearts,
        value: 5,
    };
    let c2 = enums::PokerCard {
        suit: enums::Pokersuit::Diamonds,
        value: 10,
    };
    print_card(c1);
    print_card(c2);
    println!("{:?}", enums::plus(Some((5)), Some((10))));
    print_separator();
    // array_example();
    string_array();
    two_deminsion_array();
    print_separator();
    let condition = 12 % 7 == 0;
    ifelse_example(condition);
    forloop_example();
    forloop_mutable_example();
    count_100();
    enumerate_example();
}
