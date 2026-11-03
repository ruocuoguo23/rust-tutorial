// 问题：select 超时胜出后，请求 Future 怎样被取消？
// 输入与状态：oneshot 确认本地模拟请求已开始，定时分支随后胜出，Cleanup 被 Drop。
// 结果证据：输出显示资源释放先于超时处理；测试区分拥有与借用 Future，并确认已有修改不回滚。
// 改一处：将临时 Future 改为外部 pin 后的 &mut Future，观察 select 结束不等于本体被销毁。
// 边界：没有真实网络调用；select 不创建线程；丢弃 JoinHandle 不会自动停止后台任务。
// 运行：cargo run --locked --example 08-select-cancellation

use std::time::Duration;
use tokio::sync::oneshot;

struct Cleanup;
impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("请求 Future 的局部资源被释放");
    }
}

async fn send_request(started: oneshot::Sender<()>) -> &'static str {
    let _cleanup = Cleanup;
    println!("开始请求（演示中没有实际网络调用）");
    started.send(()).unwrap();
    std::future::pending::<&'static str>().await
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let (started_tx, started_rx) = oneshot::channel();
    tokio::select! {
        result = send_request(started_tx) => {
            println!("请求完成：{result}");
        }
        _ = async {
            started_rx.await.unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
        } => {
            println!("超时分支被选中");
        }
    }
    println!("select 已结束");
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::sync::oneshot;

    struct DropProbe(Arc<AtomicUsize>);
    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn owned_losing_future_drops_state_without_rolling_back_prior_effects() {
        let drops = Arc::new(AtomicUsize::new(0));
        let effects = Arc::new(AtomicUsize::new(0));
        let drop_counter = Arc::clone(&drops);
        let effect_counter = Arc::clone(&effects);
        let (started, observed) = oneshot::channel();
        let request = async move {
            let _guard = DropProbe(drop_counter);
            effect_counter.fetch_add(1, Ordering::SeqCst);
            started.send(()).unwrap();
            std::future::pending::<()>().await;
            effect_counter.fetch_add(1, Ordering::SeqCst);
        };
        tokio::select! {
            _ = request => unreachable!("request remains pending"),
            _ = observed => {}
        }
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(effects.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn borrowed_losing_future_survives_until_its_owner_drops_it() {
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let counter = Arc::clone(&drops);
            let (started, observed) = oneshot::channel();
            let request = async move {
                let _guard = DropProbe(counter);
                started.send(()).unwrap();
                std::future::pending::<()>().await;
            };
            tokio::pin!(request);
            tokio::select! {
                _ = &mut request => unreachable!("request remains pending"),
                _ = observed => {}
            }
            assert_eq!(drops.load(Ordering::SeqCst), 0);
        } // 外部 Future 在这里销毁，不是 drop 那个 Pin<&mut _> 就销毁本体。
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
