pub mod event_queue;
pub mod types;
pub mod widget_queue;
pub mod worker_pool;

#[cfg(test)]
mod tests {
    use crate::{types::*, worker_pool::*};

    use super::*;
    use std::{any::Any, sync::Arc};
    use tokio::sync::{broadcast, mpsc};

    // 创建工作属性
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum TestWkrPpty {
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

    impl EventTypeTrait for TestEvntTp {}

    // 创建事件实体
    pub struct TestEvnt {
        time_stamp: u64,
        evnt_ppty: TestEvntTp,
    }

    impl TimeEventTrait for TestEvnt {
        type TimestampType = u64;
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
        time_stamp: u64,
        wdgt_ppty: TestWkrPpty,
    }

    impl TimeEventTrait for TestWdgt {
        type TimestampType = u64;
        fn time_stamp(&self) -> Self::TimestampType {
            self.time_stamp
        }
    }
}
