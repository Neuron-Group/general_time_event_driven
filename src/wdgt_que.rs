use crate::{tp_traits::*, types::*};
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WdgtHeap<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT>(
    BinaryHeap<Reverse<BoxdWdgt<Ft, EvntTp, WkrPpty>>>,
);

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> WdgtHeap<Ft, EvntTp, WkrPpty> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, wdgt: Box<dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty>>) {
        self.0.push(Reverse(wdgt));
    }

    pub fn pop(&mut self) -> Option<Box<dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty>>> {
        self.0.pop().map(|v| v.0)
    }

    pub fn peek(&self) -> Option<&dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty>> {
        self.0.peek().map(|v| &*v.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Default for WdgtHeap<Ft, EvntTp, WkrPpty> {
    fn default() -> Self {
        Self::new()
    }
}
