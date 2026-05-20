trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait ShapeName: Shape {
    fn name(&self) -> &'static str;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl ShapeName for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl ShapeName for Rectangle {
    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl ShapeName for Square {
    fn name(&self) -> &'static str {
        "Square"
    }
}

// fn get_perimeter_area_generic<T: Shape + ShapeName>(shape: &T) -> (String, f64, f64) {
//     (shape.name().to_string(), shape.area(), shape.perimeter())
// }

fn get_perimeter_area_generic<T>(shape: &T) -> (String, f64, f64) 
where T: Shape+ShapeName 
{
    (shape.name().to_string(), shape.area(), shape.perimeter())
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    let square = Square { side: 3.0 };

    let (name, area, perimeter) = get_perimeter_area_generic(&circle);
    println!("{} area: {:.2}", name, area);
    println!("{} perimeter: {:.2}", name, perimeter);

    let (name, area, perimeter) = get_perimeter_area_generic(&rectangle);
    println!("{} area: {:.2}", name, area);
    println!("{} perimeter: {:.2}", name, perimeter);

    let (name, area, perimeter) = get_perimeter_area_generic(&square);
    println!("{} area: {:.2}", name, area);
    println!("{} perimeter: {:.2}", name, perimeter);
}