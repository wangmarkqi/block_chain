use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::ask::Snapshot;
use crate::raft::config::*;
use crate::raft::node::Node;
use crate::trans::req_tls::{select_inner_trans, MSG};
use std::collections::{HashMap, HashSet};
use uuid::Version::Mac;

// 一旦发现心跳超时，或者找不到最近收到心跳时间，把role改成candidate

// RequestVote RPC，由candidate发送，要求其他节点选举其为leader的RPC
pub fn request_vote() {
    let urls = read_peers_url_without_self();
    let total = urls.len() as f32;
    let selfurl = Node::self_url();
    if total == 0.0 {
        // 在不启动ca的情况下，一个人启动会自己成为leader，然后就不会去找peers了,在生产环境中，应该禁止一个人成为leader
        //         sled_db::insert(DB_LEADER_URL, "");
        //         sled_db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);
        sled_db::insert(DB_LEADER_URL, &selfurl);
        sled_db::insert(DB_SELF_ROLE, ROLE_LEADER);
        return;
    }
    let a = Snapshot::new_ask(ASK_REQ_VOTE);
    let mut agree = 1 as f32;

    for url in urls.iter() {
        if let Ok(m) = a.send(url) {
            if m.body == YES {
                agree = agree + 1.0;
            };
        }
    }

    // 在不启动ca的情况下，一个人启动会自己成为leader，然后就不会去找peers了,在生产环境中，应该禁止一个人成为leader,把等号去掉就好了
    if agree >= (total + 1.0) * 0.5 {
        sled_db::insert(DB_LEADER_URL, &selfurl);
        sled_db::insert(DB_SELF_ROLE, ROLE_LEADER);
    } else {
        sled_db::insert(DB_LATEST_HEART_BEAT, &time_now_str());
        sled_db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);
    }
}
