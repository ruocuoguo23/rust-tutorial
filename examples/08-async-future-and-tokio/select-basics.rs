// 问题：怎样等待两个结果，并处理先就绪的一个分支？
// 输入与状态：左通道已有 7，右通道为空且发送端仍存活；只有左分支能立即交付消息。
// 结果证据：选择得到 (left, Some(7))；测试分别交换左右就绪状态并检查关闭结果。
// 改一处：向右通道也预先发送消息，观察不能再承诺一定选左分支。
// 边界：select! 在当前任务中推进分支，不新建任务；本例不演示超时或取消清理。
// 运行：cargo run --locked --example 08-select-basics

use tokio::sync::mpsc;

async fn select_message(
    mut left: mpsc::Receiver<u8>,
    mut right: mpsc::Receiver<u8>,
) -> (&'static str, Option<u8>) {
    tokio::select! {
        message = left.recv() => ("left", message),
        message = right.recv() => ("right", message),
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let (left_sender, left) = mpsc::channel(1);
    let (_right_sender, right) = mpsc::channel(1);
    left_sender.send(7).await.unwrap();
    let selected = select_message(left, right).await;
    assert_eq!(selected, ("left", Some(7)));
    println!("Selected: {selected:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn only_ready_branch_is_selected_on_either_side() {
        for ready_left in [true, false] {
            let (left_sender, left) = mpsc::channel(1);
            let (right_sender, right) = mpsc::channel(1);
            if ready_left {
                left_sender.send(7).await.unwrap();
            } else {
                right_sender.send(8).await.unwrap();
            }
            let selected = select_message(left, right).await;
            let expected = if ready_left {
                ("left", Some(7))
            } else {
                ("right", Some(8))
            };
            assert_eq!(selected, expected);
        }
    }

    #[tokio::test]
    async fn closed_channel_is_also_a_ready_result() {
        let (left_sender, left) = mpsc::channel(1);
        let (_right_sender, right) = mpsc::channel(1);
        drop(left_sender);
        assert_eq!(select_message(left, right).await, ("left", None));
    }
}
