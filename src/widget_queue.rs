use crate::types::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WidgetHeap<Widget: WidgetTrait>(BinaryHeap<Reverse<Widget>>);

impl<Widget: WidgetTrait + Ord> WidgetHeap<Widget> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, widget: Widget) {
        self.0.push(Reverse(widget));
    }

    pub fn pop(&mut self) -> Option<Widget> {
        self.0.pop().map(|v| v.0)
    }

    pub fn peek(&self) -> Option<&Widget> {
        self.0.peek().map(|v| &v.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Widget: WidgetTrait + Ord> Default for WidgetHeap<Widget> {
    fn default() -> Self {
        Self::new()
    }
}
