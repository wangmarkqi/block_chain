use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::candidate_do::*;
use crate::raft::config::*;
use crate::raft::follower_do::*;
use crate::raft::leader_do::{check_nodes, send_heart_beat};
use crate::raft::node::Node;
use crate::trans::req_tls::{select_inner_trans, MSG};
use std::collections::{HashMap, HashSet};
use std::thread;

pub fn run_thread() {
    // 初始化角色，leader

    init_args();

    let ticks = ConfigRaft::new().heartbeat_tick as u64;
    let d = std::time::Duration::new(ticks, 0);
    loop {
        std::thread::sleep(d);
        let self_role = sled_db::get_or_empty(DB_SELF_ROLE);
        let leader_url = sled_db::get_or_empty(DB_LEADER_URL);
        dbg!(&self_role);
        dbg!(&leader_url);
        // 在不启动ca的情况下，一个人启动会自己成为leader，然后就不会去找peers了,在生产环境中，应该禁止一个人成为leader
        if self_role == ROLE_FOLLOWER && leader_url == "".to_owned() {
            find_leader_and_record();
        };

        if self_role == ROLE_FOLLOWER {
            check_heart_beat();
        }
        if self_role == ROLE_CANDIDATE {
            request_vote();
        }

        if self_role == ROLE_LEADER {
            send_heart_beat();
            check_nodes();
        }
    }
}

// 最开始，角色follower，领导url“”
fn init_args() {
    sled_db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);
    sled_db::insert(DB_PEERS_URL, "");
    sled_db::insert(DB_LEADER_URL, "");
    sled_db::insert(DB_LATEST_HEART_BEAT, &time_now_str());
    sled_db::insert(DB_NODES, "");

    let mut l = HashSet::new();
    // 把自己的url加入并更新数据库
    let _friends_url = get_dot_env("FRIENDS");
    let friends_url: Vec<String> = _friends_url.split(",").map(|s| s.to_string()).collect();
    for i in friends_url {
        l.insert(i);
    }
    let h = sled_db::update_set_from_db_and_set(DB_PEERS_URL, l);
    if !h {
        panic!("can not init,friends url set wrong");
    }
}
