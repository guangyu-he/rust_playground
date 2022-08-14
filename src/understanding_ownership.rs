fn main() {
    {
        // a variable scope
        let s = "hello";
        println!("{}", s);
    }

    // let s = "world";
    // outside the scope there is no string
    // println!("{}", s);

    // using the string type, different memory and allocation strategies
    let mut b = String::from("hello");
    b.push_str("world");
    println!("{}", b);


    let s1 = String::from("hello");
    let s2 = s1;  // s1 variable is moved to s2, making it no longer a valid variable
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();  // s2 is a clone of s1, it is working but may resource-consuming
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;  // stuck-only data, integer value has it's own size
    println!("x = {}, y = {}", x, y);
}