pub fn init() {
    println!("{}", file!());
}
pub fn greet(name: String) {
    println!("Hello, {}!", name);
}

pub fn print_chinese() {
    let t = "hi";
    let s = "中国";
    // one chinese letter equals 3 bytes, so the range is 0..3, not 0..1
    println!("{},{}", &t[0..2], &s[0..6]);
}

pub fn first_word(s: &String) -> &str {
    let character = &s[0..1];
    character
}

pub fn push_string_example() {
    let mut s = String::from("Hello,");
    s.push_str("rust");
    println!("after first push:{:?}", s);
    s.push('!');
    println!("after second push:{:?}", s);
    s.insert(6, ' ');
    println!("after insert:{:?}", s);
    s.insert_str(7, "best ");
    println!("after insert string:{:?}", s);
    // replace will create a new string, not change the original string
    let s1 = s.replace("Hello", "Welcome");
    println!("after replace:{:?}", s1);
    let mut s2 = s1.replacen("rust", "RUST", 1);
    println!("after replace count:{:?}", s2);
    s2.replace_range(7..9, " ");
    println!("after replace range:{:?}", s2);
}

pub fn delete_string_example() {
    let mut s = String::from("Hello, rust!");
    s.pop();
    println!("after pop:{:?}", s);
    s = "你好, rust!".to_string();
    // remove the second chinese character, which is 3 bytes
    s.remove(3);
    println!("after remove:{:?}", s);
    s.truncate(3);
    println!("after truncate:{:?}", s);
    s.clear();
    println!("after clear:{:?}", s);
}

pub fn concatenate_string_example() {
    let s1 = String::from("Hello,");
    let s2 = String::from(" rust!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("Concatenated string: {}", s3);
    // both &str and String can be concatenated with + operator, but the left operand must be a String, and the right operand must be a &str
    let s1 = "welcome,";
    let s2 = " rust!";
    let s3 = format!("{}{}", s1, s2);
    println!("Concatenated string with format!: {}", s3);
}

pub fn ascii_example() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
}

pub fn unicode_example() {
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );
}

pub fn chars_example() {
    let s = "Hello, 世界!";
    for c in s.chars() {
        println!("{}", c);
    }
}
