fn main() -> () {
    let n: i32 = roman_to_int(String::from("MCMXXI"));
    println!("{}", n);
}

fn roman_to_int(s: String) -> i32 {

    let mut sum:i32 = 0;
    let mut last:i32 = 0;
    for c in s.chars().rev() {
        let current:i32 = roman_char_to_int(c);
        if current >= last{
            sum += current;
        }else {
            sum -= current;
        }
        last = current;
    }
    sum
}

fn roman_char_to_int(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}