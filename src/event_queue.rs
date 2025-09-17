use crate::types::*;
use async_priority_queue::PriorityQueue;
use std::{cmp::Reverse, sync::Arc};
// use tokio::sync::Mutex;

struct EventPipe<Event: EventTrait + Ord>(PriorityQueue<Reverse<Event>>);

impl<Event: EventTrait + Ord> EventPipe<Event> {
    fn new() -> Self {
        EventPipe::<Event>(PriorityQueue::new())
    }
}

pub struct Sender<Event: EventTrait + Ord> {
    socket: Arc<EventPipe<Event>>,
}

pub struct Receiver<Event: EventTrait + Ord> {
    socket: Arc<EventPipe<Event>>,
}

pub fn channel<Event: EventTrait + Ord>() -> (Sender<Event>, Receiver<Event>) {
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

impl<Event: EventTrait + Ord> Sender<Event> {
    pub async fn send(&self, event: Event) {
        self.socket.as_ref().0.push(Reverse(event));
    }
}

impl<Event: EventTrait + Ord> Receiver<Event> {
    pub async fn recv(&self) -> Event {
        let Reverse(result) = self.socket.as_ref().0.pop().await;
        result
    }
}
