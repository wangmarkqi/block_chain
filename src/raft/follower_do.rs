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

// talks:从env文件出发，不断更新累加数据库peers——url，找到领导的url结束。由于接受的一方会增加发送的url列表和发送方本身，所以，url会传播到网络.talks初始化了role的数据库值follower

//  role: ROLE_FOLLOWER.to_owned(),
// 一开始出了个evn读出friend，然后发送takls，不断更新数据库的peers——url，发送接受方不断累加url，发送方找到leader url，不在talking
// 找到leader就update peer，看leader url是否空

// 找领导，更新peers
pub fn find_leader_and_record() -> bool {
    let peers_url = read_peers_url_without_self();
    let leader_find = false;
    for url in peers_url {
        if let Ok(res) = _find_leader_and_record(&url) {
            if res {
                sled_db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);
                break;
            }
        }
    }
    leader_find
}

fn _find_leader_and_record(url: &str) -> Result<bool, MyError> {
    let snap = Snapshot::new_ask(ASK_SNAPSHOT);
    let res = snap.send(url)?;
    let snap_from_out: Snapshot = serde_json::from_str(&res.body)?;
    sled_db::update_set_from_db_and_set(DB_PEERS_URL, snap_from_out.peers_url);
    if snap_from_out.node.role == ROLE_LEADER {
        sled_db::insert(DB_LEADER_URL, &snap.node.url);
        return Ok(true);
    };
    if snap_from_out.leader_url == "" {
        return Ok(false);
    };
    let res2 = snap.send(&snap_from_out.leader_url)?;
    let snap_from_leader: Snapshot = serde_json::from_str(&res2.body)?;
    sled_db::update_set_from_db_and_set(DB_PEERS_URL, snap_from_leader.peers_url);
    if snap_from_leader.node.role == ROLE_LEADER {
        sled_db::insert(DB_LEADER_URL, &snap.node.url);
        return Ok(true);
    };
    Ok(false)
}
pub fn check_heart_beat() {
    // 初始化角色，leader
    let last = sled_db::get_or_empty(DB_LATEST_HEART_BEAT);
    if last == "" {
        sled_db::insert(DB_LEADER_URL, "");
        sled_db::insert(DB_SELF_ROLE, ROLE_CANDIDATE);
        return;
    }
    let differ = time_differ(&last);
    let election_elapse = ConfigRaft::new().election_tick;

    if differ > election_elapse {
        sled_db::insert(DB_LEADER_URL, "");
        sled_db::insert(DB_SELF_ROLE, ROLE_CANDIDATE);
    }
}
