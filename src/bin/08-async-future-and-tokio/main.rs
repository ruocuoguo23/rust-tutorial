#[path = "../../../examples/08-async-future-and-tokio/lazy-future-await.rs"]
mod lazy_future_await;

#[path = "../../../examples/08-async-future-and-tokio/join-futures.rs"]
mod join_futures;

#[path = "../../../examples/08-async-future-and-tokio/spawn-task.rs"]
mod spawn_task;

#[path = "../../../examples/08-async-future-and-tokio/mpsc-basics.rs"]
mod mpsc_basics;

#[path = "../../../examples/08-async-future-and-tokio/oneshot-basics.rs"]
mod oneshot_basics;

#[path = "../../../examples/08-async-future-and-tokio/select-basics.rs"]
mod select_basics;

#[path = "../../../examples/08-async-future-and-tokio/spawn-blocking.rs"]
mod spawn_blocking;

#[path = "../../../examples/08-async-future-and-tokio/detach-abort.rs"]
mod detach_abort;

#[path = "../../../examples/08-async-future-and-tokio/send-future.rs"]
mod send_future;

#[path = "../../../examples/08-async-future-and-tokio/poll-waker.rs"]
mod poll_waker;

#[path = "../../../examples/08-async-future-and-tokio/pin-unpin.rs"]
mod pin_unpin;

#[path = "../../../examples/08-async-future-and-tokio/select-cancellation.rs"]
mod select_cancellation;

#[path = "../../../examples/08-async-future-and-tokio/channel-cancellation.rs"]
mod channel_cancellation;

fn main() {
    println!("=== 入门：一次理解一个动作 ===");
    lazy_future_await::main();
    join_futures::main();
    spawn_task::main();
    mpsc_basics::main();
    oneshot_basics::main();
    select_basics::main();
    spawn_blocking::main();
    println!("=== 深入：生命周期、类型约束与内部机制 ===");
    detach_abort::main();
    send_future::main();
    poll_waker::main();
    pin_unpin::main();
    select_cancellation::main();
    channel_cancellation::main();
}
