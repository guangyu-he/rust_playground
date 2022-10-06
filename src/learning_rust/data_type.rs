fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    print_int(five_hundred, six_point_four);


    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    print_array(&mut a);
}

fn print_array(arr: &mut [i32]) {
    println!("{}", arr[0]);
}

fn print_int(x: i32, y: f64) {
    let x = if x % 10 == 0 {
        x + 1
    }else {
        x
    };
    println!("{}, {}", x, y);
}