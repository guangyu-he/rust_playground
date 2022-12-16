struct Rectangle {
    x: f32,
    y: f32,
}

impl Rectangle {
    fn new(x_input: f32, y_input: f32) -> Rectangle {
        return Rectangle { x: x_input, y: y_input };
    }

    fn rect_area(self) -> f32 {
        return self.x * self.y;
    }
}


fn main() {
    let rect: Rectangle = Rectangle::new(1.0, 2.0);
    let area = rect.rect_area();
    println!("area: {:?}", area);
}