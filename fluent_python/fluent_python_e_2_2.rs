fn a_func(a: i32, b: i32, rest: &[i32]) -> (i32, i32, &[i32]) {
    (a, b, rest)
}

fn main() {
    println!("{:?}", a_func(1, 2, &[3, 4, 5]));
}