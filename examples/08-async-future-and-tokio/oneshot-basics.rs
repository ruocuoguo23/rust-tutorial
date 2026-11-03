// 问题：怎样把一次计算结果交给等待它的人？
// 输入与状态：oneshot 发送端交付 7，接收端 await 得到这一个结果。
// 结果证据：正常接收为 7；测试另覆盖发送端提前退出和接收端提前退出。
// 改一处：把 sender.send(7) 换成 drop(sender)，观察 receiver.await 返回错误。
// 边界：oneshot 只交付一次值；send 是同步调用，不需要 await，也不证明业务结果正确。
// 运行：cargo run --locked --example 08-oneshot-basics

use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let (sender, receiver) = oneshot::channel();
    sender.send(7).unwrap();
    let value = receiver.await.unwrap();
    assert_eq!(value, 7);
    println!("One result: {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn result_is_received_once() {
        let (sender, receiver) = oneshot::channel();
        sender.send(7).unwrap();
        assert_eq!(receiver.await.unwrap(), 7);
    }

    #[tokio::test]
    async fn sender_exit_is_reported_to_receiver() {
        let (sender, receiver) = oneshot::channel::<u8>();
        drop(sender);
        assert!(receiver.await.is_err());
    }

    #[test]
    fn receiver_exit_returns_the_unsent_value() {
        let (sender, receiver) = oneshot::channel();
        drop(receiver);
        assert_eq!(sender.send(7), Err(7));
    }
}
