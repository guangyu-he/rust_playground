fn main() {
    let a_message = "abc";

    match a_message {
        "ab" => println!("ab"),
        "bc" => println!("bc"),
        "abc" => println!("abc"),
        _ => panic!("haha"),
    }
}