use async_priority_queue::PriorityQueue;

use tokio::time::{Duration, sleep};

use general_time_event_driven::{types::*, worker_pool::*};
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
#[derive(Debug)]
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
        if (-1..=1).contains(&relative_time) {
            return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                id: self.id,
                judge_type: JudgeType::CriticalPerfect,
            })));
        }
        if (-5..=5).contains(&relative_time) {
            return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                id: self.id,
                judge_type: JudgeType::Perfect,
            })));
        }
        if (-20..=20).contains(&relative_time) {
            return RuntimeState::Ready(RuntimeEvent::Some(Box::new(RtV {
                id: self.id,
                judge_type: JudgeType::Good,
            })));
        }
        RuntimeState::Ready(RuntimeEvent::Missed)
    }
}

fn test_judge_perfect() {
    let mut widget = TestWdgt {
        time_stamp: 100,
        wdgt_ppty: TestWkrPpty::Wkr0,
        id: 2,
    };
    let event = TestEvnt {
        time_stamp: 121, // 相对时间+5
        evnt_ppty: TestEvntTp::All,
    };
    let event_boxed: Box<dyn EventTrait<EventType = TestEvntTp, TimestampType = i64>> =
        Box::new(event);
    let result = widget.judge(&event_boxed);
    if let RuntimeState::Ready(RuntimeEvent::Some(rtv)) = result {
        // let rtv = rtv.judge_type;
        println!("{:#?}", rtv.judge_type);
        matches!(rtv.judge_type, JudgeType::Perfect);
    } else {
        panic!("Expected Perfect result");
    }
}

async fn test_worker_pool_process() {
    // 创建工作属性配置：Worker0使用ProcessOnce模式，处理Wkr0Only事件
    let boxed_closure: Box<dyn Fn(&TestEvntTp) -> bool + Send + Sync> =
        Box::new(|event_type: &TestEvntTp| matches!(event_type, TestEvntTp::Wkr0Only));
    let worker_properties = vec![(TestWkrPpty::Wkr0, WorkerMode::ProcessOnce, boxed_closure)];

    // 创建测试组件
    let widgets: Vec<
        Box<
            dyn WidgetTrait<
                    TimestampType = i64,
                    EventType = TestEvntTp,
                    WorkerProperty = TestWkrPpty,
                    ReturnType = RtV,
                >,
        >,
    > = vec![Box::new(TestWdgt {
        time_stamp: 100, // println!("1");
        wdgt_ppty: TestWkrPpty::Wkr0,
        id: 1,
    })];

    // 构建WorkerPool
    let (event_sender, mut runtime_event_receiver, _worker_pool) =
        WorkerPool::<i64, TestEvntTp, TestWkrPpty, RtV>::build(worker_properties, widgets).await;

    // 发送测试事件

    let test_event: Box<dyn EventTrait<EventType = TestEvntTp, TimestampType = i64> + 'static> =
        Box::new(TestEvnt {
            time_stamp: 120,
            evnt_ppty: TestEvntTp::Wkr0Only,
        });

    event_sender.send(test_event).await;

    // 验证处理结果
    if let Some(RuntimeEvent::Some(result)) = runtime_event_receiver.recv().await {
        println!("{:#?}", result.judge_type);
    } else {
        panic!("WorkerPool未正确处理事件");
    }
}
#[tokio::main]
async fn main() {
    test_judge_perfect();
    test_worker_pool_process().await;
}
