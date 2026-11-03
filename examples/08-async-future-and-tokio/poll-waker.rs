// 问题：返回 Pending 的 Future 如何让执行器以后继续 poll？
// 输入与状态：YieldOnce 第一次将状态 false 改为 true，并唤醒 Waker；第二次返回 Ready(17)。
// 结果证据：await 得到 17；手动 poll 测试同时断言第一次是 Pending 且发出一次唤醒。
// 改一处：把初始状态 false 改为 true，观察第一次 poll 就返回 Ready，不再需要唤醒。
// 边界：Waker 只是请求继续调度；本例不实现执行器，完成后的 Future 不应随意再次 poll。
// 运行：cargo run --locked --example 08-poll-waker

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct YieldOnce(pub bool);

impl Future for YieldOnce {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            return Poll::Ready(17);
        }
        self.0 = true;
        context.waker().wake_by_ref(); // Request another poll; no busy-loop executor needed.
        Poll::Pending
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P1: Poll / Waker");
    assert_eq!(YieldOnce(false).await, 17);
    println!("YieldOnce: Pending + wake -> Ready(17)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::task::{Wake, Waker};

    struct WakeCount(AtomicUsize);

    impl Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn pending_requests_a_wake_before_the_next_poll_completes() {
        let wakes = Arc::new(WakeCount(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&wakes));
        let mut context = Context::from_waker(&waker);
        let mut future = YieldOnce(false);
        assert_eq!(Pin::new(&mut future).poll(&mut context), Poll::Pending);
        assert_eq!(wakes.0.load(Ordering::Relaxed), 1);
        assert_eq!(Pin::new(&mut future).poll(&mut context), Poll::Ready(17));
        // Do not poll a completed Future unless its API explicitly permits it.
    }

    #[tokio::test]
    async fn runtime_drives_waking_future() {
        assert_eq!(YieldOnce(false).await, 17);
    }
}
