use crate::tp_traits::*;
use std::{cmp::Ordering, hash::Hash};

/// 时间戳事件基础Trait
///
/// 所有事件和组件的基础Trait，提供时间戳获取功能
pub trait TimeEvntT: Send + Sync {
    type TmStmpTp: Ord;
    fn time_stamp(&self) -> Self::TmStmpTp;
}

// 为时间戳事件实现比较方法
impl<TmStmpTp: Ord> PartialEq for dyn TimeEvntT<TmStmpTp = TmStmpTp> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TmStmpTp: Ord> Eq for dyn TimeEvntT<TmStmpTp = TmStmpTp> {}

impl<TmStmpTp: Ord> PartialOrd for dyn TimeEvntT<TmStmpTp = TmStmpTp> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TmStmpTp: Ord> Ord for dyn TimeEvntT<TmStmpTp = TmStmpTp> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 组件Trait，继承自TimeEvntT
///
/// 定义组件的基本行为，包括工作属性获取和事件判断
pub trait WdgtT: Send + Sync + TimeEvntT {
    type EvntTp: EvntTpT;
    type WkrPpty: WkrPptyT;
    fn get_wkr_ppt(&self) -> Self::WkrPpty;
    fn judge(
        &self,
        evnt: &BoxdEvnt<Self::TmStmpTp, Self::EvntTp>,
    ) -> RtStt<Self::TmStmpTp, Self::EvntTp, Self::WkrPpty>;
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> PartialEq for dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Eq for dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty> {}

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> PartialOrd for dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> Ord for dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 事件Trait，继承自TimeEvntT
///
/// 定义事件的基本行为，提供事件属性获取功能
pub trait EvntT: Send + Sync + TimeEvntT {
    type EvntTp: EvntTpT;
    fn get_evnt_ppt(&self) -> Self::EvntTp;
}
impl<TmStmpTp: Ord, EvntTp: EvntTpT> PartialEq for dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT> Eq for dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp> {}

impl<TmStmpTp: Ord, EvntTp: EvntTpT> PartialOrd for dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TmStmpTp: Ord, EvntTp: EvntTpT> Ord for dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 返回值Trait
///
/// 作为通用返回值的标记Trait
pub trait RtvT: Send + Sync {}

/// Box智能指针包装的返回值类型
pub type BoxdRtvt = Box<dyn RtvT>;

pub enum RtStt<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> {
    Pending(BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>, RtEvnt),
    Ready(RtEvnt),
}

/// 运行时事件枚举
///
/// 表示处理后的结果事件
pub enum RtEvnt {
    /// 包含返回值的事件
    Some(BoxdRtvt),
    /// 错过处理的事件
    Missed,
}

/// 事件类型Trait
///
/// 事件类型的标记Trait，用于泛型约束
pub trait EvntTpT {}

/// 工作属性Trait
///
/// 工作属性的标记Trait，要求实现基本的哈希和比较功能
pub trait WkrPptyT: Eq + Hash + Clone + Send + Sync {}

pub type BoxdEvnt<TmStmpTp, EvntTp> = Box<dyn EvntT<TmStmpTp = TmStmpTp, EvntTp = EvntTp>>;
pub type BoxdWdgt<TmStmpTp, EvntTp, WkrPpty> =
    Box<dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty>>;

/// 工作模式枚举
///
/// 定义工作线程的事件处理模式
pub enum WkrMod {
    /// 单次处理模式
    PrcsOnce,
    /// 多次处理模式
    PrcsMltiTimes,
}
