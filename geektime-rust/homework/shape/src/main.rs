
struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

trait CalShape {
    fn square(&self) -> f64;
    fn circumference(&self) -> f64;
}

impl CalShape for Rectangle {
    fn square(&self) -> f64 {
        self.a * self.b
    }
    fn circumference(&self) -> f64 {
        self.a * 2f64 + self.b * 2f64
    }
}

impl CalShape for Circle {
    fn square(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
    fn circumference(&self) -> f64 {
        std::f64::consts::PI * 2f64 * self.r
    }
}

impl CalShape for Triangle {
    fn square(&self) -> f64 {
        self.a * self.b * self.c / 2f64
    }
    fn circumference(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn main() {
    let shape = Shape::Rectangle(Rectangle{a: 10f64, b: 20f64});
    match shape {
        Shape::Rectangle(rec) => {
            println!("{}", rec.square());
        },
        Shape::Circle(cir) => {
            println!("{}", cir.circumference());
        },
        Shape::Triangle(tri) => {
            println!("{}", tri.circumference());
        },
    };
}
