use async_priority_queue::PriorityQueue;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let queue = PriorityQueue::new();
    let arcd_queue = Arc::new(queue);

    let queue_ = arcd_queue.clone();
    let mut handle_vec = vec![];
    handle_vec.push(tokio::spawn(async move {
        for i in 1..100 {
            queue_.push(i);
            sleep(Duration::from_secs(1)).await;
        }
    }));
    handle_vec.push(tokio::spawn(async move {
        let queue_ = arcd_queue.clone();
        for _ in 1..100 {
            println!("{}", queue_.pop().await);
        }
    }));

    for h in handle_vec.into_iter() {
        _ = h.await;
    }
}
