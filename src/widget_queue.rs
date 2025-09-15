use crate::types::*;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WdgtHeap<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT>(
    BinaryHeap<Reverse<BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>>>,
);

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> WdgtHeap<TmStmpTp, EvntTp, WkrPpty> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(
        &mut self,
        wdgt: Box<dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty>>,
    ) {
        self.0.push(Reverse(wdgt));
    }

    pub fn pop(
        &mut self,
    ) -> Option<Box<dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty>>> {
        self.0.pop().map(|v| v.0)
    }

    pub fn peek(
        &self,
    ) -> Option<&dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty>> {
        self.0.peek().map(|v| &*v.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Default
    for WdgtHeap<TmStmpTp, EvntTp, WkrPpty>
{
    fn default() -> Self {
        Self::new()
    }
}
