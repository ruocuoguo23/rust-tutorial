// 问题：顺序 await 和 join! 怎样组合两个异步计算？
// 输入与状态：同一个 message 函数先顺序等待，再由 join! 在当前任务内共同推进。
// 结果证据：两种写法均得到 A 和 B；测试用相互传信验证 join 可以推进两条分支。
// 改一处：把测试中的 join! 改成先 await first 再 await second，观察两者互相等待。
// 边界：join! 不提交独立 Task；不使用耗时或完成日志顺序证明并行和性能。
// 运行：cargo run --locked --example 08-join-futures

use std::time::Duration;

async fn message(name: &'static str) -> &'static str {
    println!("开始 {name}");
    tokio::time::sleep(Duration::from_millis(1)).await;
    println!("完成 {name}");
    name
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("顺序 await：");
    let first = message("A").await;
    let second = message("B").await;
    assert_eq!((first, second), ("A", "B"));
    println!("Sequential results: {first}, {second}");

    println!("join!：");
    let (first, second) = tokio::join!(message("A"), message("B"));
    assert_eq!((first, second), ("A", "B"));
    println!("Joined results: {first}, {second}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn join_advances_both_branches_without_spawning() {
        let (started, observed) = tokio::sync::oneshot::channel();
        let (release, released) = tokio::sync::oneshot::channel();
        let first = async {
            started.send(()).unwrap();
            released.await.unwrap();
            1
        };
        let second = async {
            observed.await.unwrap();
            release.send(()).unwrap();
            2
        };
        let joined = async { tokio::join!(first, second) };
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), joined)
                .await
                .unwrap(),
            (1, 2)
        );
    }
}
