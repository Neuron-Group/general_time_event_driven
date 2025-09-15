use crate::types::*;
use async_priority_queue::PriorityQueue;
use std::{cmp::Reverse, sync::Arc};
// use tokio::sync::Mutex;

struct EvntPip<TmStmpTp: Ord, EvntTp: EvntTpT>(
    PriorityQueue<Reverse<Box<dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp>>>>,
);

impl<TmStmpTp: Ord, EvntTp: EvntTpT> EvntPip<TmStmpTp, EvntTp> {
    fn new() -> Self {
        EvntPip::<TmStmpTp, EvntTp>(PriorityQueue::new())
    }
}

pub struct Sndr<TmStmpTp: Ord, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<TmStmpTp, EvntTp>>,
}

pub struct Rcvr<TmStmpTp: Ord, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<TmStmpTp, EvntTp>>,
}

pub fn chnl<TmStmpTp: Ord, EvntTp: EvntTpT>() -> (Sndr<TmStmpTp, EvntTp>, Rcvr<TmStmpTp, EvntTp>) {
    let evnt_pip = EvntPip::new();
    let arcd_evnt_pip = Arc::new(evnt_pip);
    (
        Sndr {
            socket: arcd_evnt_pip.clone(),
        },
        Rcvr {
            socket: arcd_evnt_pip,
        },
    )
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT> Sndr<TmStmpTp, EvntTp> {
    pub async fn send(&self, evnt: Box<dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp>>) {
        self.socket.as_ref().0.push(Reverse(evnt));
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT> Rcvr<TmStmpTp, EvntTp> {
    pub async fn recv(&self) -> Box<dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp>> {
        let Reverse(result) = self.socket.as_ref().0.pop().await;
        result
    }
}
