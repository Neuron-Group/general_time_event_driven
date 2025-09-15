use crate::{type_traits::*, types::*};
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

pub struct Sender<Ft: FloatTrait, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<Ft, EvntTp>>,
}

pub struct Receiver<Ft: FloatTrait, EvntTp: EvntTpT> {
    socket: Arc<EvntPip<Ft, EvntTp>>,
}

pub fn make_channel<Ft: FloatTrait, EvntTp: EvntTpT>() -> (Sender<Ft, EvntTp>, Receiver<Ft, EvntTp>) {
    let evnt_pip = EvntPip::new();
    let arcd_evnt_pip = Arc::new(evnt_pip);
    (
        Sender {
            socket: arcd_evnt_pip.clone(),
        },
        Receiver {
            socket: arcd_evnt_pip,
        },
    )
}

impl<Ft: FloatTrait, EvntTp: EvntTpT> Sender<Ft, EvntTp> {
    pub async fn send(&self, evnt: Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>>) {
        self.socket.as_ref().0.push(Reverse(evnt));
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT> Receiver<Ft, EvntTp> {
    pub async fn recv(&self) -> Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>> {
        let Reverse(result) = self.socket.as_ref().0.pop().await;
        result
    }
}
