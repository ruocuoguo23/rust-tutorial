#[path = "../../../examples/07-concurrency-and-synchronization/thread-ownership-scope.rs"]
mod thread_ownership_scope;

#[path = "../../../examples/07-concurrency-and-synchronization/send-sync.rs"]
mod send_sync;

#[path = "../../../examples/07-concurrency-and-synchronization/mutex-basics.rs"]
mod mutex_basics;

#[path = "../../../examples/07-concurrency-and-synchronization/mutex-counter.rs"]
mod mutex_counter;

#[path = "../../../examples/07-concurrency-and-synchronization/bounded-channel.rs"]
mod bounded_channel;

#[path = "../../../examples/07-concurrency-and-synchronization/rwlock.rs"]
mod rwlock;

#[path = "../../../examples/07-concurrency-and-synchronization/atomic-updates.rs"]
mod atomic_updates;

#[path = "../../../examples/07-concurrency-and-synchronization/memory-ordering.rs"]
mod memory_ordering;

#[path = "../../../examples/07-concurrency-and-synchronization/mutex-guard-invariants.rs"]
mod mutex_guard_invariants;

#[path = "../../../examples/07-concurrency-and-synchronization/logical-race.rs"]
mod logical_race;

#[path = "../../../examples/07-concurrency-and-synchronization/lock-order.rs"]
mod lock_order;

#[path = "../../../examples/07-concurrency-and-synchronization/shared-store.rs"]
mod shared_store;

fn main() {
    println!("基础：线程、能力要求与访问共享数据");
    thread_ownership_scope::main();
    send_sync::main();
    mutex_basics::main();
    mutex_counter::main();
    bounded_channel::main();
    rwlock::main();
    atomic_updates::main();
    memory_ordering::main();
    println!("综合与边界：完整动作、竞态、锁顺序与 Store");
    mutex_guard_invariants::main();
    logical_race::main();
    lock_order::main();
    shared_store::main();
}
