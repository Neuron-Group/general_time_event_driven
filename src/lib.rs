pub mod evnt_que;
pub mod tp_traits;
pub mod types;
pub mod wdgt_que;
pub mod wkr_pool;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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
