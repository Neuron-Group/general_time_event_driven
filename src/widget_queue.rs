use crate::types::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WidgetHeap<
    TimestampType: Ord,
    EventType: EventTypeTrait,
    WorkerProperty: WorkerPropertyTrait,
    ReturnType: ReturnTypeTrait,
>(BinaryHeap<Reverse<BoxedWidget<TimestampType, EventType, WorkerProperty, ReturnType>>>);

impl<
    TimestampType: Ord,
    EventType: EventTypeTrait,
    WorkerProperty: WorkerPropertyTrait,
    ReturnType: ReturnTypeTrait,
> WidgetHeap<TimestampType, EventType, WorkerProperty, ReturnType>
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
                    ReturnType = ReturnType,
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
                    ReturnType = ReturnType,
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
            ReturnType = ReturnType,
        >,
    > {
        self.0.peek().map(|v| &*v.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<
    TimestampType: Ord,
    EventType: EventTypeTrait,
    WorkerProperty: WorkerPropertyTrait,
    ReturnType: ReturnTypeTrait,
> Default for WidgetHeap<TimestampType, EventType, WorkerProperty, ReturnType>
{
    fn default() -> Self {
        Self::new()
    }
}
