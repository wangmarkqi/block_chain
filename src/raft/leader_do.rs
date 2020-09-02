use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::ask::Snapshot;
use crate::raft::config::*;
use crate::raft::node::Node;
use crate::trans::req_tls::{select_inner_trans, MSG};
use std::collections::{HashMap, HashSet};

pub fn send_heart_beat() {
    let peers = read_peers_url_without_self();
    for url in peers {
        let res = _send_heart_beat(&url);
        match res {
            Ok(_b) => check_url_validity(&url, true),
            Err(_e) => check_url_validity(&url, false),
        }
    }
}

pub fn check_url_validity(url: &str, success: bool) {
    if success {
        let a = 0;
        sled_db::insert(url.clone(), &a.to_string());
        return;
    }
    let del_times = ConfigRaft::new().respose_del_times as i32;
    let before = sled_db::get_or_empty(url);
    if before == "" {
        let a = 1;
        sled_db::insert(url, &a.to_string());
        return;
    }
    let before_number = before.parse::<i32>().unwrap();
    let now_number = before_number + 1;
    if now_number > del_times {
        let _b = sled_db::del_str_from_set(DB_PEERS_URL, &url);
        // 删除完以后清零，不然永远加不上去了
        let a = 0;
        sled_db::insert(url.clone(), &a.to_string());
    } else {
        sled_db::insert(url.clone(), &now_number.to_string());
    };
}

pub fn _send_heart_beat(url: &str) -> Result<bool, MyError> {
    // 如果不是leader，直接返回
    // new ask include leader node
    let snap = Snapshot::new_ask(ASK_SNAPSHOT);
    let res = snap.send(url)?;
    let snap_from_out: Snapshot = serde_json::from_str(&res.body)?;
    sled_db::update_set_from_db_and_set(DB_PEERS_URL, snap_from_out.peers_url);
    update_nodes(snap_from_out.node);
    Ok(true)
}

pub fn check_nodes() {
    let nodes = read_nodes();
    let max = Block::max().id;
    for (_id, node) in nodes.iter() {
        if node.block.id >= max {
            continue;
        };
        let res = ask_save_block(node.clone(), max);
        match res {
            Ok(s) => println!("check nodes res{}", s),
            Err(e) => println!("check nodes err{}", e),
        }
    }
}

pub fn ask_save_block(node: Node, max: u64) -> Result<bool, MyError> {
    let out = node.block.id;
    let start = out + 1;
    let end = max + 1;
    let mut snap = Snapshot::new_ask(ASK_SAVE_BLOCK);
    let mut v = vec![];
    for id in start..end {
        let b = Block::get_block(id);
        v.push(b);
    }
    let res = serde_json::to_string(&v)?;
    snap.value = res;
    let res2 = snap.send(&node.url)?;
    if res2.body == YES {
        return Ok(true);
    }
    Ok(false)
}
