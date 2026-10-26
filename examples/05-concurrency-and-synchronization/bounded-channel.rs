// 问题：有界 Channel 怎样转移消息所有权，何时阻塞或结束？
// 输入与状态：容量 1 的 channel 按顺序传 one、two；另一个 receiver 提前 drop。
// 结果证据：先 drain 再 join 得到两条消息；断开时 SendError 返回 unsent。
// 改一处：把 producer.join 移到 drain 前，分析队列满时双方为何可能互相等待。
// 边界：send 消费非 Copy 消息；迭代结束需要所有 sender 都被释放。
// 运行：cargo run --locked --example 05-bounded-channel

use std::sync::mpsc;
use std::thread;

fn channel_messages() -> Vec<String> {
    let (sender, receiver) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        let message = String::from("one");
        sender.send(message).unwrap();
        // println!("{message}"); // E0382: send moved this non-Copy value.
        sender.send(String::from("two")).unwrap();
    });
    // Drain before join: the producer may be waiting for capacity.
    // The last sender drops when the producer exits, so iteration can finish.
    let received = receiver.into_iter().collect();
    producer.join().unwrap();
    received
}

fn undelivered_message() -> String {
    let (sender, receiver) = mpsc::sync_channel(1);
    drop(receiver);
    sender.send(String::from("unsent")).unwrap_err().0
}

pub fn main() {
    println!("P0: bounded Channel / ownership");
    let messages = channel_messages();
    let returned = undelivered_message();
    assert_eq!(messages, ["one", "two"]);
    assert_eq!(returned, "unsent");
    println!("Owned messages: {messages:?}; disconnected send returns: {returned}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_transfers_messages_and_finishes_after_sender_drop() {
        assert_eq!(channel_messages(), ["one", "two"]);
    }

    #[test]
    fn disconnected_receiver_returns_the_undelivered_message() {
        assert_eq!(undelivered_message(), "unsent");
    }
}
