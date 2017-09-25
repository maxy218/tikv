// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use prometheus::*;

lazy_static! {
    pub static ref PD_SEND_MSG_HISTOGRAM: Histogram =
        register_histogram!(
            "tikv_pd_msg_send_duration_seconds",
            "Bucketed histogram of PD message send duration",
             exponential_buckets(0.0005, 10.0, 7).unwrap()
        ).unwrap();

    pub static ref PD_REQ_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_request_sent_total",
            "Total number of pd client request sent.",
            &["type", "status"]
        ).unwrap();

    pub static ref PD_HEARTBEAT_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_heartbeat_sent_total",
            "Total number of raftstore pd client heartbeat sent.",
            &["type"]
        ).unwrap();

    pub static ref PD_VALIDATE_PEER_COUNTER_VEC: CounterVec =
        register_counter_vec!(
            "tikv_raftstore_pd_validate_peer_total",
            "Total number of raftstore pd worker validate peer task.",
            &["type"]
        ).unwrap();
    
    pub static ref STORE_SIZE_GAUGE_VEC: GaugeVec =
        register_gauge_vec!(
            "tikv_raftstore_store_size_bytes",
            "Size of raftstore storage.",
            &["type"]
        ).unwrap();
}
