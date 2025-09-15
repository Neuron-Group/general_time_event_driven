pub mod event_queue;
pub mod type_traits;
pub mod types;
pub mod widget_queue;
pub mod worker_pool;

#[cfg(test)]
mod tests {
    use crate::types::{EvntTpT, TimeEvntT, WkrPptyT};

    use super::*;
    use std::sync::Arc;
    use tokio::sync::{broadcast, mpsc};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestEvntTp(String);
    impl EvntTpT for TestEvntTp {}

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestWkrPpty(String);
    impl WkrPptyT for TestWkrPpty {}

    #[derive(Debug, Clone)]
    struct TestEvnt {
        time_stamp: f64,
        evnt_ppt: TestEvntTp,
        id: u32,
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
