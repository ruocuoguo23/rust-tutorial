#[path = "../../../examples/06-async-future-and-tokio/lazy-future-await.rs"]
mod lazy_future_await;

#[path = "../../../examples/06-async-future-and-tokio/spawn-blocking.rs"]
mod spawn_blocking;

#[path = "../../../examples/06-async-future-and-tokio/poll-waker.rs"]
mod poll_waker;

#[path = "../../../examples/06-async-future-and-tokio/pin-unpin.rs"]
mod pin_unpin;

#[path = "../../../examples/06-async-future-and-tokio/send-future.rs"]
mod send_future;

#[path = "../../../examples/06-async-future-and-tokio/select-cancellation.rs"]
mod select_cancellation;

#[path = "../../../examples/06-async-future-and-tokio/channel-cancellation.rs"]
mod channel_cancellation;

#[path = "../../../examples/06-async-future-and-tokio/detach-abort.rs"]
mod detach_abort;

fn main() {
    lazy_future_await::main();
    spawn_blocking::main();
    poll_waker::main();
    pin_unpin::main();
    send_future::main();
    select_cancellation::main();
    channel_cancellation::main();
    detach_abort::main();
}
