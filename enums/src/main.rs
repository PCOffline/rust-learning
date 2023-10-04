enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimiter(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => radius * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(width, height) => width * 2.0 + height * 2.0,
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn main() {
    let circle = Shape::Circle(0.5);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(1.0, 2.0, 3.0);

    println!("The perimiter of circle is {}", circle.get_perimiter());
    println!("The perimiter of rectangle is {}", rectangle.get_perimiter());
    println!("The perimiter of triangle is {}", triangle.get_perimiter());
}
