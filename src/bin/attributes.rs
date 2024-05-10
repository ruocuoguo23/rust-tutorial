#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// we can override the default implementation of PartialEq trait
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

// example of use of Clone, Debug and PartialEq traits
fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = point1.clone(); // Clone trait allows us to clone the struct
    let point3 = Point { x: 1, y: 3 };

    // Debug trait allows us to print the struct
    println!("point1: {:?}", point1);

    // PartialEq trait allows us to compare the struct
    if point1 == point2 {
        println!("The points are equal.");
    }

    if point1 == point3 {
        println!("The points are equal, point1: {:?}, point3: {:?}", point1, point3);
    }
}
