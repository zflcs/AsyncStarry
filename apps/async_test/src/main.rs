#![no_std]
#![no_main]

#[macro_use]
extern crate axstd as std;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::thread;
use std::{sync::Arc, vec::Vec};

use std::os::arceos::api::task::{self as api, AxWaitQueueHandle};

const NUM_DATA: usize = 2_000_000;
const NUM_TASKS: usize = 16;

async fn barrier() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static BARRIER_WQ: AxWaitQueueHandle = AxWaitQueueHandle::new();
    static BARRIER_COUNT: AtomicUsize = AtomicUsize::new(0);

    BARRIER_COUNT.fetch_add(1, Ordering::Relaxed);
    api::ax_async_wait_queue_wait(
        &BARRIER_WQ,
        || BARRIER_COUNT.load(Ordering::Relaxed) == NUM_TASKS,
        None,
    ).await;
    api::ax_wait_queue_wake(&BARRIER_WQ, u32::MAX); // wakeup all
}



fn sqrt(n: &u64) -> u64 {
    let mut x = *n;
    loop {
        if x * x <= *n && (x + 1) * (x + 1) > *n {
            return x;
        }
        x = (x + *n / x) / 2;
    }
}

#[axstd::async_main]
#[no_mangle]
async fn main() {
    let mut rng = SmallRng::seed_from_u64(0xdead_beef);
    let vec = Arc::new(
        (0..NUM_DATA)
            .map(|_| rng.next_u32() as u64)
            .collect::<Vec<_>>(),
    );
    let expect: u64 = vec.iter().map(sqrt).sum();


    // equals to sleep(500ms)
    let timeout = api::ax_async_wait_queue_wait(
        &AxWaitQueueHandle::new(),
        || false,
        Some(std::time::Duration::from_millis(500)),
    ).await;
    assert!(timeout);

    let mut tasks = Vec::with_capacity(NUM_TASKS);
    for i in 0..NUM_TASKS {
        let vec = vec.clone();
        tasks.push(thread::spawn(move || async move {
            let left = i * (NUM_DATA / NUM_TASKS);
            let right = (left + (NUM_DATA / NUM_TASKS)).min(NUM_DATA);
            println!(
                "part {}: {:?} [{}, {})",
                i,
                thread::current().id(),
                left,
                right
            );

            let partial_sum: u64 = vec[left..right].iter().map(sqrt).sum();
            barrier().await;

            println!("part {}: {:?} finished", i, thread::current().id());
            partial_sum
        }));
    }

    // let actual = tasks.into_iter().map(|t| t.join().await.unwrap()).sum();
    let mut actual = 0;
    for t in tasks {
        actual += t.join().await.unwrap();
    }
    println!("sum = {}", actual);
    assert_eq!(expect, actual);

    println!("Parallel summation tests run OK!");
    0
}
