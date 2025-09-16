pub mod event_queue;
pub mod types;
pub mod widget_queue;
pub mod worker_pool;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::{types::*, worker_pool::*};
    use super::*;
    use std::{any::Any, sync::Arc};
    use tokio::sync::{broadcast, mpsc};

    // 创建工作属性
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TestWkrPpty {
        Wkr0,
        Wkr1,
    }

    impl WorkerPropertyTrait for TestWkrPpty {}

    // 创建事件类型
    #[derive(Clone, Copy)]
    pub enum TestEvntTp {
        Wkr0Only,
        All,
    }

    // 创建判定类型
    pub enum JudgeType {
        CriticalPerfect,
        Perfect,
        Good,
    }

    // 创建返回类型
    pub struct RtV {
        id: usize,
        judge_type: JudgeType,
    }

    impl ReturnTypeTrait for RtV {}

    impl EventTypeTrait for TestEvntTp {}

    // 创建事件实体
    pub struct TestEvnt {
        time_stamp: i64,
        evnt_ppty: TestEvntTp,
    }

    impl TimeEventTrait for TestEvnt {
        type TimestampType = i64;
        fn time_stamp(&self) -> Self::TimestampType {
            self.time_stamp
        }
    }

    impl EventTrait for TestEvnt {
        type EventType = TestEvntTp;
        fn get_event_property(&self) -> Self::EventType {
            self.evnt_ppty
        }
    }

    // 创建组件实体
    pub struct TestWdgt {
        time_stamp: i64,
        wdgt_ppty: TestWkrPpty,
        id: usize,
    }

    impl TimeEventTrait for TestWdgt {
        type TimestampType = i64;
        fn time_stamp(&self) -> Self::TimestampType {
            self.time_stamp
        }
    }

    impl WidgetTrait for TestWdgt {
        type EventType = TestEvntTp;
        type WorkerProperty = TestWkrPpty;
        type ReturnType = RtV;
        fn get_worker_property(&self) -> Self::WorkerProperty {
            self.wdgt_ppty
        }
        fn judge(
            &mut self,
            event: &BoxedEvent<Self::TimestampType, Self::EventType>,
        ) -> RuntimeState<Self::ReturnType> {
            if event.time_stamp() > self.time_stamp + 40 {
                return RuntimeState::Ready(RuntimeEvent::Missed);
            }
            let relative_time = self.time_stamp + 20 - event.time_stamp();
            if relative_time >= -1 && relative_time <= 1 {
                return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                    id: self.id,
                    judge_type: JudgeType::CriticalPerfect,
                })));
            }
            if relative_time >= -5 && relative_time <= 5 {
                return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                    id: self.id,
                    judge_type: JudgeType::Perfect,
                })));
            }
            if relative_time >= -20 && relative_time <= 20 {
                return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                    id: self.id,
                    judge_type: JudgeType::Good,
                })));
            }
            RuntimeState::Ready(RuntimeEvent::Missed)
        }
    }

    #[test]
    fn test_judge_perfect() {
        let mut widget = TestWdgt {
            time_stamp: 100,
            wdgt_ppty: TestWkrPpty::Wkr0,
            id: 2,
        };
        let event = TestEvnt {
            time_stamp: 125, // 相对时间+5
            evnt_ppty: TestEvntTp::All,
        };
        let event_boxed: Box<dyn EventTrait<EventType = TestEvntTp, TimestampType = i64>> =
            Box::new(event);
        let result = widget.judge(&event_boxed);
        if let RuntimeState::Ready(RuntimeEvent::Some(rtv)) = result {
            // let rtv = rtv.judge_type;
            matches!(rtv.judge_type, JudgeType::Perfect);
        } else {
            panic!("Expected Perfect result");
        }
    }
}
