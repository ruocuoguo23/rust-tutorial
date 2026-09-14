mod cancellation;
mod polling;
mod send;

use std::cell::Cell;
use std::marker::PhantomPinned;

async fn lazy_value(calls: &Cell<u8>) -> u8 {
    calls.set(calls.get() + 1);
    tokio::task::yield_now().await;
    7
}

fn pin_and_unpin() {
    struct Marker {
        value: u8,
        _pin: PhantomPinned,
    }
    let pinned = Box::pin(Marker {
        value: 7,
        _pin: PhantomPinned,
    });
    let address = &*pinned as *const Marker;
    let moved_handle = pinned;
    assert!(std::ptr::eq(address, &*moved_handle));
    println!(
        "Pin: handle moved, pointee unchanged, value={}",
        moved_handle.value
    );
    // std::pin::Pin::into_inner(moved_handle); // E0277: Marker is !Unpin.
    let unpinned = std::pin::Pin::into_inner(Box::pin(7_u8)); // u8: Unpin.
    assert_eq!(*unpinned, 7);
    // Marker is not self-referential; PhantomPinned demonstrates the type constraint only.
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("P0: Future / async / await / Executor / Tokio");
    let calls = Cell::new(0);
    let future = lazy_value(&calls);
    assert_eq!(calls.get(), 0);
    println!("Constructed Future: calls={}", calls.get());
    let value = future.await;
    assert_eq!(value, 7);
    assert_eq!(calls.get(), 1);
    println!("After await: calls={}, result={value}", calls.get());
    let result = tokio::spawn(async { 6 * 7 }).await.unwrap();
    let blocking = tokio::task::spawn_blocking(|| (1..=10).sum::<u64>())
        .await
        .unwrap();
    assert_eq!((result, blocking), (42, 55));
    println!("spawn: {result}; spawn_blocking: {blocking}");

    println!("P1: Poll / Waker / Pin / Unpin / Send");
    assert_eq!(polling::YieldOnce(false).await, 17);
    println!("YieldOnce: Pending + wake -> Ready(17)");
    pin_and_unpin();
    send::run().await;

    println!("P1: Bounded channel / cancellation / detach / abort");
    println!(
        "Drained channel: {:?}",
        cancellation::bounded_messages().await
    );
    println!(
        "Cancelled send drops: {}",
        cancellation::cancelled_send_drops_its_message().await
    );
    println!(
        "Cancelled reserve retains: {}",
        cancellation::cancelled_reservation_keeps_the_message().await
    );
    println!(
        "Detached task result: {}",
        cancellation::detached_task_completes().await
    );
    println!(
        "Prior effects after abort: {}",
        cancellation::aborted_task_keeps_prior_effects().await
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn async_body_runs_only_when_polled() {
        let calls = Cell::new(0);
        let future = lazy_value(&calls);
        assert_eq!(calls.get(), 0);
        assert_eq!(future.await, 7);
        assert_eq!(calls.get(), 1);
        drop(lazy_value(&calls));
        assert_eq!(calls.get(), 1); // Never-polled Future caused no body side effect.
    }

    #[tokio::test]
    async fn runtime_drives_waking_and_send_examples() {
        assert_eq!(polling::YieldOnce(false).await, 17);
        send::run().await;
        pin_and_unpin();
    }
}
