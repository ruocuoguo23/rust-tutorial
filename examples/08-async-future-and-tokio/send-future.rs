// 问题：Future 为什么可能不是 Send，缩短局部值的作用域能否改变结果？
// 输入与状态：先在块内读取 Rc，再离开块并 await；Rc 不留在跨 await 保存的状态中。
// 结果证据：assert_send 在编译时检查 Future: Send，然后实际 await 完成。
// 改一处：按注释中的写法，让 Rc 留到 await 后使用，再调用 assert_send，观察编译拒绝。
// 边界：Send 取决于 Future 捕获及跨挂起点保存的状态；move 捕获不自动使 Future 成为 Send。
// 运行：cargo run --locked --example 08-send-future

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

async fn run() {
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

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P1: Send Future");
    run().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn shortened_scope_produces_a_send_future() {
        run().await;
    }
}
