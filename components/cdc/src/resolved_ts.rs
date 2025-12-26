// Copyright 2025 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::BTreeMap;

use collections::HashMap;
use txn_types::TimeStamp;

use crate::delegate::DownstreamId;

struct Resolver {
    region_id: u64,

    locks_by_key: HashMap<Arc[u8], Timestamp>,

    lock_ts_heap: BTreeMap<TimeStamp, Key>
}

impl Resolver {
    pub fn new(region_id: u64) -> Resolver {
        region_id
    }

    pub fn add_downstream(region_id: u64, downstream_id: DownstreamId) {

    }

    pub fn remove_downstream(region_id: u64, downstream_id: DownstreamId) {

    }

    pub fn resolved_ts(&self) -> Timestamp {
    }

    pub fn track_lock(&mut self, start_ts: Timestamp, key: Vec<u8>, generation: u64){

    }

    pub fn untrack_lock(&mut self, key: Vec<u8>) {

    }

}

impl Drop for Resolver {
    fn drop(&mut self) {

    }
}