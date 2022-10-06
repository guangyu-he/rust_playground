fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3, 5]));
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut sum: i32 = 0;
    if digits.is_empty() {
        vec![1]
    } else {
        for i in 0..digits.len() {
            sum += digits[i] * i32::pow(10, digits.len() as u32 - i as u32 - 1 as u32);
        }
        sum += 1;
        let sum_string: String = String::from(sum.to_string());
        let mut output: Vec<i32> = Vec::new();
        for item in sum_string.chars() {
            output.push(item as i32 - 48);
        }
        output
    }
}
