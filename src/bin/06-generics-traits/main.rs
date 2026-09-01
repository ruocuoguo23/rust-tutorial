#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

trait Encoder {
    fn encode(&self) -> String;
}

impl Encoder for u32 {
    fn encode(&self) -> String {
        self.to_string()
    }
}

impl Encoder for bool {
    fn encode(&self) -> String {
        self.to_string()
    }
}

fn encode_static<T: Encoder>(value: &T) -> String {
    value.encode() // The concrete type is known at this call site.
}

fn encode_dynamic(value: &dyn Encoder) -> String {
    value.encode() // The implementation is selected through a trait object.
}

fn main() {
    let number = 42_u32;
    println!("Static dispatch: {}", encode_static(&number));

    let values: Vec<Box<dyn Encoder>> = vec![Box::new(7_u32), Box::new(true)];
    for value in values {
        println!("Dynamic dispatch: {}", encode_dynamic(value.as_ref()));
    }

    let point1 = Point { x: 1, y: 2 };
    let point2 = point1.clone();
    let point3 = Point { x: 1, y: 3 };
    println!("Derived Debug and Clone: {point1:?}, {point2:?}");
    println!("Manual PartialEq compares x only: {}", point1 == point3);
}
