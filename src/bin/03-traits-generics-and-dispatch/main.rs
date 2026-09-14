use std::fmt;

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

trait Source {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    next: u8,
    end: u8,
}

impl Source for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }
        let value = self.next;
        self.next += 1; // next < end <= u8::MAX, so increment cannot overflow.
        Some(value)
    }
}

fn one<S: Source<Item = u8>>(source: &mut S) -> Option<u8> {
    source.next()
}

fn items() -> impl Iterator<Item = u8> {
    0..3 // Return-position impl Trait hides this single concrete Range type.
}

struct Label(String);

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

fn main() {
    println!("P0: Generic / Trait Bound / impl Trait / dyn Trait");
    let number = 42_u32;
    println!("Static dispatch: {}", encode_static(&number));
    println!("Borrowed dynamic dispatch: {}", encode_dynamic(&number));
    // &dyn Encoder itself does not require a new heap allocation.

    let values: Vec<Box<dyn Encoder>> = vec![Box::new(7_u32), Box::new(true)];
    for value in values {
        println!("Dynamic dispatch: {}", encode_dynamic(value.as_ref()));
    }

    println!("P0: Associated Type");
    let mut counter = Counter { next: 0, end: 2 };
    assert_eq!(one(&mut counter), Some(0));
    assert_eq!(one(&mut counter), Some(1));
    assert_eq!(one(&mut counter), None);
    println!(
        "Source<Item=u8>: 0, 1, None; impl Iterator: {:?}",
        items().collect::<Vec<_>>()
    );

    println!("P1: Newtype / From / Into / fallible conversion");
    let label: Label = "rust".into(); // From provides the corresponding Into implementation.
    println!("Local newtype Display: {label}");
    assert!(u8::try_from(256_u16).is_err());
    // impl Display for Vec<String> { ... } // E0117: both trait and Self are foreign.
    // A type alias would not create the local identity that Label provides.

    let point1 = Point { x: 1, y: 2 };
    let point2 = point1.clone();
    let point3 = Point { x: 1, y: 3 };
    println!("Derived Debug and Clone: {point1:?}, {point2:?}");
    println!("Manual PartialEq compares x only: {}", point1 == point3);
    println!(
        "Different y values do not affect this equality: {}, {}",
        point1.y, point3.y
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_paths_preserve_the_encoder_contract() {
        assert_eq!(encode_static(&42_u32), "42");
        assert_eq!(encode_dynamic(&42_u32), "42");
        assert_eq!(encode_static(&false), "false");
        assert_eq!(encode_dynamic(&true), "true");
        assert_eq!(items().collect::<Vec<_>>(), [0, 1, 2]);
    }

    #[test]
    fn source_stays_exhausted_and_handles_the_u8_boundary() {
        let mut source = Counter {
            next: 254,
            end: 255,
        };
        assert_eq!(one(&mut source), Some(254));
        assert_eq!(one(&mut source), None);
        assert_eq!(one(&mut source), None);
        assert_eq!(one(&mut Counter { next: 0, end: 0 }), None);
        assert_eq!(one(&mut Counter { next: 2, end: 1 }), None);
    }

    #[test]
    fn newtype_conversion_and_manual_equality_keep_their_declared_meaning() {
        let label: Label = "rust".into();
        assert_eq!(label.to_string(), "rust");
        assert_eq!(Label::from("").to_string(), "");
        assert_eq!(Point { x: 1, y: 2 }, Point { x: 1, y: 3 });
        assert_ne!(Point { x: 1, y: 2 }, Point { x: 2, y: 2 });
    }
}
