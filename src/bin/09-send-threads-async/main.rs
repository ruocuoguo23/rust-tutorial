use std::future;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn assert_send<T: Send>(_: T) {}

async fn send_after_shortening_scope() {
    {
        let local = Rc::new(7);
        println!("Local value before await: {local}");
    } // Rc is dropped before the suspension point.
    future::ready(()).await;
}

fn main() {
    // Calling an async function only constructs its lazy Future. This assertion consumes it.
    assert_send(send_after_shortening_scope());
    println!("Future is Send because Rc does not cross await");

    let shared = Arc::new(7);
    let worker_value = Arc::clone(&shared);
    let result = thread::spawn(move || *worker_value + 1).join().unwrap();
    println!("Arc moved across thread: {result}");

    // Keeping Rc across await would make the Future non-Send:
    // async fn not_send() {
    //     let value = Rc::new(7);
    //     future::ready(()).await;
    //     println!("{value}");
    // }
    // assert_send(not_send()); // future cannot be sent between threads safely.
}
