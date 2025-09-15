use crate::{tp_traits::*, types::*};
use async_priority_queue::PriorityQueue;
use std::{cmp::Reverse, sync::Arc};
// use tokio::sync::Mutex;

struct EvntPip<Ft: FloatTrait, EvntTp: EvntTpT>(
    PriorityQueue<Reverse<Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>>>>,
);

impl<Ft: FloatTrait, EvntTp: EvntTpT> EvntPip<Ft, EvntTp> {
    fn new() -> Self {
        EvntPip::<Ft, EvntTp>(PriorityQueue::new())
    }
}

pub struct Sndr<Ft: FloatTrait, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<Ft, EvntTp>>,
}

pub struct Rcvr<Ft: FloatTrait, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<Ft, EvntTp>>,
}

pub fn chnl<Ft: FloatTrait, EvntTp: EvntTpT>() -> (Sndr<Ft, EvntTp>, Rcvr<Ft, EvntTp>) {
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

impl<Ft: FloatTrait, EvntTp: EvntTpT> Sndr<Ft, EvntTp> {
    pub async fn send(&self, evnt: Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>>) {
        self.socket.as_ref().0.push(Reverse(evnt));
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT> Rcvr<Ft, EvntTp> {
    pub async fn recv(&self) -> Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>> {
        let Reverse(result) = self.socket.as_ref().0.pop().await;
        result
    }
}
