fn main() -> () {
    println!("{}", is_palindrome(121_i32))
}

fn is_palindrome(x: i32) -> bool {
    let string: String = String::from(x.to_string());
    let mut p_string: String = String::from("");
    for c in string.chars().rev() {
        p_string.push(c);
    }
    if string == p_string {
        true
    } else {
        false
    }
}