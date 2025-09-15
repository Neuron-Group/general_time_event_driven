use crate::tp_traits::*;
use std::{cmp::Ordering, hash::Hash};

// 时间戳事件基础角色
pub trait TimeEvntT: Send + Sync {
    type Ft: FloatTrait;
    fn time_stamp(&self) -> Self::Ft;
}

// 为时间戳事件实现比较方法
impl<Ft: FloatTrait> PartialEq for dyn TimeEvntT<Ft = Ft> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<Ft: FloatTrait> Eq for dyn TimeEvntT<Ft = Ft> {}

impl<Ft: FloatTrait> PartialOrd for dyn TimeEvntT<Ft = Ft> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Ft: FloatTrait> Ord for dyn TimeEvntT<Ft = Ft> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

// 组件角色继承于时间戳事件
pub trait WdgtT: Send + Sync + TimeEvntT {
    type EvntTp: EvntTpT;
    type WkrPpty: WkrPptyT;
    fn get_wkr_ppt(&self) -> Self::WkrPpty;
    fn judge(
        &self,
        evnt: &BoxdEvnt<Self::Ft, Self::EvntTp>,
    ) -> RtStt<Self::Ft, Self::EvntTp, Self::WkrPpty>;
}

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> PartialEq for dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Eq for dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty> {}

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> PartialOrd for dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Ord for dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

// 事件角色继承于时间戳事件
pub trait EvntT: Send + Sync + TimeEvntT {
    type EvntTp: EvntTpT;
    fn get_evnt_ppt(&self) -> Self::EvntTp;
}
impl<Ft: FloatTrait, EvntTp: EvntTpT> PartialEq for dyn EvntT<Ft = Ft, EvntTp = EvntTp> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT> Eq for dyn EvntT<Ft = Ft, EvntTp = EvntTp> {}

impl<Ft: FloatTrait, EvntTp: EvntTpT> PartialOrd for dyn EvntT<Ft = Ft, EvntTp = EvntTp> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Ft: FloatTrait, EvntTp: EvntTpT> Ord for dyn EvntT<Ft = Ft, EvntTp = EvntTp> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}
pub trait RtvT: Send + Sync {}

pub type BoxdRtvt = Box<dyn RtvT>;

pub enum RtStt<Ft: FloatTrait, EvntTp: EvntTpT, WkrPpty: WkrPptyT> {
    Pending(BoxdWdgt<Ft, EvntTp, WkrPpty>, RtEvnt),
    Ready(RtEvnt),
}

pub enum RtEvnt {
    Some(BoxdRtvt),
    Missed,
}

pub trait EvntTpT {}

pub trait WkrPptyT: Eq + Hash + Clone + Send + Sync {}

pub type BoxdEvnt<Ft, EvntTp> = Box<dyn EvntT<Ft = Ft, EvntTp = EvntTp>>;
pub type BoxdWdgt<Ft, EvntTp, WkrPpty> =
    Box<dyn WdgtT<Ft = Ft, EvntTp = EvntTp, WkrPpty = WkrPpty>>;

pub enum WkrMod {
    PrcsOnce,
    PrcsMltiTimes,
}
