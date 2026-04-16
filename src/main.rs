mod basics;
mod functions;
mod ownership;
mod string;
mod types;

use basics::MAX_POINTS;

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
}
