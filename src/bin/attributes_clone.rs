use std::rc::Rc;

struct BigData {
    data: Vec<i32>,
}

#[derive(Clone)] // we derive the Clone trait for SharedData
struct SharedData {
    name: String,
    big_data: Rc<BigData>,
}
fn main() {
    let big_data = Rc::new(BigData {
        data: vec![1; 100000], // it's a big data
    });

    let shared_data1 = SharedData {
        name: "Alice".to_string(),
        big_data: Rc::clone(&big_data),
    };

    let shared_data2 = shared_data1.clone();

    // the data is not cloned, only the reference count is increased
    println!("Name: {}, Data pointer: {:?}", shared_data1.name, Rc::as_ptr(&shared_data1.big_data));
    println!("Name: {}, Data pointer: {:?}", shared_data2.name, Rc::as_ptr(&shared_data2.big_data));
}