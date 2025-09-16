use crate::types::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WidgetHeap<
    TimestampType: Ord,
    EventType: EventTypeTrait,
    WorkerProperty: WorkerPropertyTrait,
>(BinaryHeap<Reverse<BoxedWidget<TimestampType, EventType, WorkerProperty>>>);

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait>
    WidgetHeap<TimestampType, EventType, WorkerProperty>
{
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(
        &mut self,
        widget: Box<
            dyn WidgetTrait<
                    TimestampType = TimestampType,
                    EventType = EventType,
                    WorkerProperty = WorkerProperty,
                >,
        >,
    ) {
        self.0.push(Reverse(widget));
    }

    pub fn pop(
        &mut self,
    ) -> Option<
        Box<
            dyn WidgetTrait<
                    TimestampType = TimestampType,
                    EventType = EventType,
                    WorkerProperty = WorkerProperty,
                >,
        >,
    > {
        self.0.pop().map(|v| v.0)
    }

    pub fn peek(
        &self,
    ) -> Option<
        &dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >,
    > {
        self.0.peek().map(|v| &*v.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait> Default
    for WidgetHeap<TimestampType, EventType, WorkerProperty>
{
    fn default() -> Self {
        Self::new()
    }
}
