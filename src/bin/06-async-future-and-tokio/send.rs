use std::future;
use std::rc::Rc;

fn assert_send<T: Send>(_: &T) {}

async fn send_after_shortening_scope() {
    {
        let local = Rc::new(7);
        println!("Local value before await: {local}");
    } // Rc is dropped before the suspension point.
    future::ready(()).await;
}

pub async fn run() {
    let future = send_after_shortening_scope();
    assert_send(&future); // Borrow to check the type, then actually drive the Future.
    future.await;
    println!("Future is Send because Rc does not cross await");

    // Keeping Rc across await would make the Future non-Send:
    // async fn not_send() {
    //     let value = Rc::new(7);
    //     future::ready(()).await;
    //     println!("{value}");
    // }
    // assert_send(&not_send()); // future cannot be sent between threads safely.
}
