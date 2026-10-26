#[path = "../../../examples/05-concurrency-and-synchronization/thread-ownership-scope.rs"]
mod thread_ownership_scope;

#[path = "../../../examples/05-concurrency-and-synchronization/send-sync.rs"]
mod send_sync;

#[path = "../../../examples/05-concurrency-and-synchronization/mutex-guard-invariants.rs"]
mod mutex_guard_invariants;

#[path = "../../../examples/05-concurrency-and-synchronization/bounded-channel.rs"]
mod bounded_channel;

#[path = "../../../examples/05-concurrency-and-synchronization/logical-race.rs"]
mod logical_race;

#[path = "../../../examples/05-concurrency-and-synchronization/lock-order.rs"]
mod lock_order;

#[path = "../../../examples/05-concurrency-and-synchronization/rwlock.rs"]
mod rwlock;

#[path = "../../../examples/05-concurrency-and-synchronization/atomic-updates.rs"]
mod atomic_updates;

#[path = "../../../examples/05-concurrency-and-synchronization/memory-ordering.rs"]
mod memory_ordering;

fn main() {
    thread_ownership_scope::main();
    send_sync::main();
    mutex_guard_invariants::main();
    bounded_channel::main();
    logical_race::main();
    lock_order::main();
    rwlock::main();
    atomic_updates::main();
    memory_ordering::main();
}
