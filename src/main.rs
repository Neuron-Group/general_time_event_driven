use async_priority_queue::PriorityQueue;

use tokio::time::{Duration, sleep};

// use general_time_event_driven::{types::*, worker_pool::*};
use std::{any::Any, sync::Arc};
use tokio::sync::{broadcast, mpsc};

fn test_judge_perfect() {}

async fn test_worker_pool_process() {}
#[tokio::main]
async fn main() {
    test_judge_perfect();
    test_worker_pool_process().await;
}
