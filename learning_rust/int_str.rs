fn main() -> () {
    let a = "1".parse::<i32>().unwrap();
    let b = a.to_string();
    println!("{:?}", b);
}
