use std::io;

fn main() {
    println!("Converting Cel to Fahrenheit...");

    let mut cel = String::new();
    io::stdin().read_line(&mut cel).expect("Failed to read line");

    let cel_input: f32 = match cel.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e)
    };

    let fahr: f32 = cel_input * 1.8 + 32.0;

    println!("{}", fahr);


}