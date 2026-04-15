pub fn init() {
    println!("{}", file!());
}
//push str
pub fn push_word(mut word: String) -> String {
    word.push_str(", world!");
    word
}
// move partially
pub fn move_part() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    // only use the reference of person.age, but move person.name
    // destructuring: {name, age} = person; --> name 2 attributes.
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's age from person struct is {}", person.age);
    //println!("The person's name from person struct is {}", person.name);
}
// compare reference
pub fn compare_ref(input: i32) {
    let x = input;
    let y = &x;
    // in println it will automatically transmit y to its reference value, but if u want the addr, use {:p}
    println!("value of x:{}, y: address:{:p}, value:{}", x, y, *y);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// check length of a String
pub fn calculate_length(s: &String) -> usize {
    s.len()
}
// changable ref (u want to modify the ref value directly)
pub fn push_ref(s: &mut String, str: &String) {
    s.push_str(str);
}

pub fn push_word_expamle() {
    let example_word = String::from("hello");
    println!("{}", push_word(example_word));
}
// if u still want to have the ownership of s
pub fn calculate_length_example() {
    let s = String::from("rust study");
    let len = calculate_length(&s);
    println!("The length of {} is {}", s, len);
}

pub fn push_ref_example() {
    let mut origin = String::from("hello");
    let insert = String::from(", world");
    push_ref(&mut origin, &insert);
    // origin = origin + insert
    println!("final result: {}", &origin);
}
