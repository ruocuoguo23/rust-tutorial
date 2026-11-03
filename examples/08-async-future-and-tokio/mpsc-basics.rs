// 问题：怎样通过 mpsc 通道发送多条消息，并知道何时读完？
// 输入与状态：容量 2 的队列依次接收 7、8，最后发送端退出后，接收方排空队列。
// 结果证据：消息按顺序收集为 [7, 8]；测试分别检查满队列、排空和接收端退出。
// 改一处：把容量改为 1，但仍先发送两条再接收，观察第二次 send 必须等容量而无法前进。
// 边界：本例只讲收发与关闭；发送成功不代表业务处理完成，取消发送另见深入例。
// 运行：cargo run --locked --example 08-mpsc-basics

use tokio::sync::mpsc;

pub async fn bounded_messages() -> Vec<u8> {
    let (sender, mut receiver) = mpsc::channel(2);
    sender.send(7).await.unwrap();
    sender.send(8).await.unwrap();
    drop(sender);

    let mut messages = Vec::new();
    while let Some(message) = receiver.recv().await {
        messages.push(message);
    }
    messages
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let messages = bounded_messages().await;
    assert_eq!(messages, [7, 8]);
    println!("Drained channel: {messages:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bounded_channel_drains_and_closes() {
        assert_eq!(bounded_messages().await, [7, 8]);
        let (sender, receiver) = mpsc::channel(1);
        drop(receiver);
        assert_eq!(sender.send(9).await.unwrap_err().0, 9);
    }

    #[tokio::test]
    async fn full_queue_returns_message_and_last_sender_controls_close() {
        let (sender, mut receiver) = mpsc::channel(1);
        sender.send(7).await.unwrap();
        assert_eq!(sender.try_send(8), Err(mpsc::error::TrySendError::Full(8)));
        let last_sender = sender.clone();
        drop(sender);
        assert_eq!(receiver.recv().await, Some(7));
        assert_eq!(receiver.try_recv(), Err(mpsc::error::TryRecvError::Empty));
        drop(last_sender);
        assert_eq!(receiver.recv().await, None);
    }
}
