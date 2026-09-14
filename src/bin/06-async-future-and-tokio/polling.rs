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
}
