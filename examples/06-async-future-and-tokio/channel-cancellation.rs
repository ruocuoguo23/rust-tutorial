// 问题：有界通道等待发送时被取消，消息的所有权会怎样？
// 输入与状态：容量 1 的队列先装满；biased select 确保发送或预留先等待，再选择取消分支。
// 结果证据：通道按序收到 [7,8]；取消 send 丢弃其拥有的消息一次，取消 reserve 保留外部消息。
// 改一处：增大容量让发送可立即完成，观察“队列已满”的前提不再成立。
// 边界：取消 reserve 会丢失排队位置；先预留容量可以保住外部消息，但不会回滚已经发生的业务副作用。
// 运行：cargo run --locked --example 06-channel-cancellation

use std::future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
struct Message(&'static str, Arc<AtomicUsize>);

impl Drop for Message {
    fn drop(&mut self) {
        self.1.fetch_add(1, Ordering::Relaxed);
    }
}

pub async fn bounded_messages() -> Vec<u8> {
    let (sender, mut receiver) = mpsc::channel(1);
    let producer = tokio::spawn(async move {
        sender.send(7).await.unwrap();
        sender.send(8).await.unwrap();
    });
    let mut messages = Vec::new();
    while let Some(message) = receiver.recv().await {
        messages.push(message);
    }
    producer.await.unwrap();
    messages
}

pub async fn cancelled_send_drops_its_message() -> usize {
    let drops = Arc::new(AtomicUsize::new(0));
    let (sender, mut receiver) = mpsc::channel(1);
    sender
        .send(Message("queued", Arc::clone(&drops)))
        .await
        .unwrap();

    tokio::select! {
        biased; // Poll the full send first, then deterministically choose cancellation.
        _ = sender.send(Message("cancelled", Arc::clone(&drops))) => unreachable!("queue is full"),
        _ = future::ready(()) => {}
    }
    let cancelled_drops = drops.load(Ordering::Relaxed);
    assert_eq!(cancelled_drops, 1); // The losing send Future owned its unsent Message.
    assert_eq!(receiver.recv().await.unwrap().0, "queued");
    assert!(matches!(
        receiver.try_recv(),
        Err(mpsc::error::TryRecvError::Empty)
    ));
    cancelled_drops
}

pub async fn cancelled_reservation_keeps_the_message() -> String {
    let (sender, mut receiver) = mpsc::channel(1);
    sender.send(String::from("queued")).await.unwrap();
    let unsent = String::from("keep me"); // Ownership stays outside the waiting Future.
    tokio::select! {
        biased;
        permit = sender.reserve() => {
            permit.unwrap().send(unsent);
            return String::from("sent");
        }
        _ = future::ready(()) => {}
    }
    assert_eq!(receiver.recv().await.as_deref(), Some("queued"));
    unsent // Cancelling reserve loses the queue position, not this value.
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P1: Bounded channel / cancellation");
    println!("Drained channel: {:?}", bounded_messages().await);
    println!(
        "Cancelled send drops: {}",
        cancelled_send_drops_its_message().await
    );
    println!(
        "Cancelled reserve retains: {}",
        cancelled_reservation_keeps_the_message().await
    );
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
    async fn send_cancellation_drops_owned_message_but_reserve_keeps_it() {
        assert_eq!(cancelled_send_drops_its_message().await, 1);
        assert_eq!(cancelled_reservation_keeps_the_message().await, "keep me");
    }
}
