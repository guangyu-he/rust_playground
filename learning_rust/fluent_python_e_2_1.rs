fn main() {
    let symbols = "!$%&/";
    let mut codes = Vec::new();
    for symbol in symbols.chars() {
        codes.push(symbol as u8);
    }
    println!("{:?}", codes);

    let codes = symbols.chars().map(|symbol| symbol as u8).collect::<Vec<u8>>();
    println!("{:?}", codes);

    let last = symbols.chars().map(|symbol| symbol as u8).last().unwrap();
    println!("{}", last);
}