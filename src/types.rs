use std::{cmp::Ordering, hash::Hash};

/// 时间戳事件基础Trait
///
/// 所有事件和组件的基础Trait，提供时间戳获取功能
pub trait TimeEventTrait: Send + Sync {
    type TimestampType: Ord;
    fn time_stamp(&self) -> Self::TimestampType;
}

// 为时间戳事件实现比较方法
impl<TimestampType: Ord> PartialEq for dyn TimeEventTrait<TimestampType = TimestampType> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TimestampType: Ord> Eq for dyn TimeEventTrait<TimestampType = TimestampType> {}

impl<TimestampType: Ord> PartialOrd for dyn TimeEventTrait<TimestampType = TimestampType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TimestampType: Ord> Ord for dyn TimeEventTrait<TimestampType = TimestampType> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 组件Trait，继承自TimeEventTrait
///
/// 定义组件的基本行为，包括工作属性获取和事件判断
pub trait WidgetTrait: Send + Sync + TimeEventTrait {
    type EventType: EventTypeTrait;
    type WorkerProperty: WorkerPropertyTrait;
    fn get_worker_property(&self) -> Self::WorkerProperty;
    fn judge(&mut self, event: &BoxedEvent<Self::TimestampType, Self::EventType>) -> RuntimeState;
}

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait> PartialEq
    for dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >
{
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait> Eq
    for dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >
{
}

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait> PartialOrd
    for dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait, WorkerProperty: WorkerPropertyTrait> Ord
    for dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 事件Trait，继承自TimeEventTrait
///
/// 定义事件的基本行为，提供事件属性获取功能
pub trait EventTrait: Send + Sync + TimeEventTrait {
    type EventType: EventTypeTrait;
    fn get_event_property(&self) -> Self::EventType;
}
impl<TimestampType: Ord, EventType: EventTypeTrait> PartialEq for dyn EventTrait<TimestampType = TimestampType, EventType = EventType> {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp() == other.time_stamp()
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait> Eq for dyn EventTrait<TimestampType = TimestampType, EventType = EventType> {}

impl<TimestampType: Ord, EventType: EventTypeTrait> PartialOrd for dyn EventTrait<TimestampType = TimestampType, EventType = EventType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TimestampType: Ord, EventType: EventTypeTrait> Ord for dyn EventTrait<TimestampType = TimestampType, EventType = EventType> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp().cmp(&other.time_stamp())
    }
}

/// 返回值Trait
///
/// 作为通用返回值的标记Trait
pub trait ReturnTypeTrait: Send + Sync {}

/// Box智能指针包装的返回值类型
pub type BoxedReturnType = Box<dyn ReturnTypeTrait>;

pub enum RuntimeState {
    Pending(RuntimeEvent),
    Ready(RuntimeEvent),
}

/// 运行时事件枚举
///
/// 表示处理后的结果事件
pub enum RuntimeEvent {
    /// 包含返回值的事件
    Some(BoxedReturnType),
    /// 错过处理的事件
    Missed,
}

/// 事件类型Trait
///
/// 事件类型的标记Trait，用于泛型约束
pub trait EventTypeTrait {}

/// 工作属性Trait
///
/// 工作属性的标记Trait，要求实现基本的哈希和比较功能
pub trait WorkerPropertyTrait: Eq + Hash + Clone + Send + Sync {}

pub type BoxedEvent<TimestampType, EventType> =
    Box<dyn EventTrait<TimestampType = TimestampType, EventType = EventType>>;
pub type BoxedWidget<TimestampType, EventType, WorkerProperty> = Box<
    dyn WidgetTrait<
            TimestampType = TimestampType,
            EventType = EventType,
            WorkerProperty = WorkerProperty,
        >,
>;

/// 工作模式枚举
///
/// 定义工作线程的事件处理模式
pub enum WorkerMode {
    /// 单次处理模式
    ProcessOnce,
    /// 多次处理模式
    ProcessMultiTimes,
}
