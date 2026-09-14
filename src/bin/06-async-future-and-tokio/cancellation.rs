use std::future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

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

pub async fn detached_task_completes() -> u8 {
    let (finish, finished) = oneshot::channel();
    let handle = tokio::spawn(async move {
        finish.send(42).unwrap();
    });
    drop(handle); // Dropping JoinHandle detaches; it does not abort the task.
    finished.await.unwrap() // Completion acknowledgement, without timing assumptions.
}

pub async fn aborted_task_keeps_prior_effects() -> usize {
    let effects = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&effects);
    let (started, ready) = oneshot::channel();
    let handle = tokio::spawn(async move {
        observed.fetch_add(1, Ordering::Relaxed);
        started.send(()).unwrap();
        future::pending::<()>().await;
        observed.fetch_add(1, Ordering::Relaxed);
    });
    ready.await.unwrap(); // Ensure the first effect happened before requesting cancellation.
    handle.abort();
    assert!(handle.await.unwrap_err().is_cancelled());
    effects.load(Ordering::Relaxed) // Cancellation stops the next step, but does not roll back 1.
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

    #[tokio::test]
    async fn detach_and_abort_have_different_effects() {
        assert_eq!(detached_task_completes().await, 42);
        assert_eq!(aborted_task_keeps_prior_effects().await, 1);
    }
}
