use std::sync::Arc;

use dashmap::DashMap;
use rtcps::{cli::PORT_RANGE, count_open_ports};

#[test]
fn test_port_count() {
    let hashmap = Arc::new(DashMap::new());

    for i in PORT_RANGE {
        hashmap.insert(i, true);
    }

    assert_eq!(count_open_ports(&hashmap), PORT_RANGE.max().unwrap())
}
