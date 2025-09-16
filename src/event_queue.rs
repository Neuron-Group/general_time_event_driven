use crate::types::*;
use async_priority_queue::PriorityQueue;
use std::{cmp::Reverse, sync::Arc};
// use tokio::sync::Mutex;

struct EventPipe<TimestampType: Ord, EventType: EventTypeTrait>(
    PriorityQueue<
        Reverse<Box<dyn EventTrait<TimestampType = TimestampType, EventType = EventType>>>,
    >,
);

impl<TimestampType: Ord, EventType: EventTypeTrait> EventPipe<TimestampType, EventType> {
    fn new() -> Self {
        EventPipe::<TimestampType, EventType>(PriorityQueue::new())
    }
}

pub struct Sender<TimestampType: Ord, EventType: EventTypeTrait> {
    socket: Arc<EventPipe<TimestampType, EventType>>,
}

pub struct Receiver<TimestampType: Ord, EventType: EventTypeTrait> {
    socket: Arc<EventPipe<TimestampType, EventType>>,
}

pub fn channel<TimestampType: Ord, EventType: EventTypeTrait>() -> (
    Sender<TimestampType, EventType>,
    Receiver<TimestampType, EventType>,
) {
    let event_pipe = EventPipe::new();
    let arc_event_pipe = Arc::new(event_pipe);
    (
        Sender {
            socket: arc_event_pipe.clone(),
        },
        Receiver {
            socket: arc_event_pipe,
        },
    )
}

impl<TimestampType: Ord, EventType: EventTypeTrait> Sender<TimestampType, EventType> {
    pub async fn send(
        &self,
        event: Box<dyn EventTrait<TimestampType = TimestampType, EventType = EventType>>,
    ) {
        self.socket.as_ref().0.push(Reverse(event));
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait> Receiver<TimestampType, EventType> {
    pub async fn recv(
        &self,
    ) -> Box<dyn EventTrait<TimestampType = TimestampType, EventType = EventType>> {
        let Reverse(result) = self.socket.as_ref().0.pop().await;
        result
    }
}
