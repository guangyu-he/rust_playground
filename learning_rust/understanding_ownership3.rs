fn old_ownership() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()会返回当前字符串的长度 
    (s, length)
}

fn new_ownership() {
    let s1 = String::from("hello");

    let len = calculate_length_new(&s1);  // using reference parameter

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_new(s: &String) -> usize {
    s.len()
}

fn main() {
    old_ownership();

    new_ownership();
}