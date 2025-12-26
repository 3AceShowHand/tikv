// Copyright 2025 TiKV Project Authors. Licensed under Apache-2.0.

use core::fmt;
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};

use collections::HashMap;
use tikv_util::memory::MemoryQuota;
use txn_types::{Key, TimeStamp};

use crate::delegate::{DownstreamId, ObservedRange};

// Resolver tracks each region's locks start-ts, the resolved-ts is calculated
// per downstream. The region may be subscribed multiple times, each time create
// one new downstream.
// * The region may contains multiple tables, subscriptions to the different
//   tables belong to the tracked by the different downstreams.
// * The table may be subscribed multiple times, also belongs to the different
//   downstreams.
struct Resolver {
    region_id: u64,

    // record each Key's start-ts.
    locks_by_key: HashMap<Key, Timestamp>,

    lock_ts_heap: HashMap<DownstreamId, BTreeMap<TimeStamp, Key>>,

    observed_ranges: HashMap<DownstreamId, ObservedRange>,

    memory_quota: Arc<MemoryQuota>,
}

impl fmt::Debug for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("resolver")
            .field("region_id", &self.id)
            .field("key_count", &self.locks_by_key.len())
            .field("downstream_count", &self.lock_ts_heap.len())
            .finish()
    }
}

impl Resolver {
    pub fn new(region_id: u64, memory_quota: Arc<MemoryQuota>) -> Resolver {
        Self {
            region_id,
            locks_by_key: HashMap::default(),
            lock_ts_heap: HashMap::default(),
            observed_ranges: HashMap::default(),
            memory_quota,
        }
    }

    pub fn add_downstream(&mut self, downstream_id: DownstreamId, range: ObservedRange) {
        self.observed_ranges.insert(downstream_id, range)
    }

    pub fn remove_downstream(&mut self, downstream_id: DownstreamId) {
        self.observed_ranges.remove(downstream_id)
    }

    pub fn resolved_ts(&self, downstream_id: DownstreamId) -> Timestamp {}

    pub fn track_lock(&mut self, start_ts: Timestamp, key: Key, generation: u64) {}

    pub fn untrack_lock(&mut self, key: Key) {}
}

impl Drop for Resolver {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {

    fn test_resolver() {}
}
