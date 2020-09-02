use crate::common::myerror::MyError;
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::node::Node;
use crate::trans::req_tls::MSG;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub trait Sender<T> {
    fn send(&self, url: &str) -> Result<T, MyError>;
}

pub trait Replier {
    fn reply(&self) -> Result<MSG, MyError>;
}

#[derive(Debug, Copy, Clone)]
pub struct ConfigRaft {
    pub election_tick: i64,
    pub heartbeat_tick: i64,
    // 找领导，和领导对peers ，对最后一个区块
    pub respose_del_times: i64,
}

impl ConfigRaft {
    pub fn new() -> ConfigRaft {
        const HEARTBEAT_TICK: i64 = 1;
        ConfigRaft {
            heartbeat_tick: HEARTBEAT_TICK,
            election_tick: 20 * HEARTBEAT_TICK,
            respose_del_times: 10 * HEARTBEAT_TICK,
        }
    }
}

pub fn read_peers_url_without_self() -> HashSet<String> {
    let self_url = Node::self_url();
    let mut res = sled_db::read_set_from_db(DB_PEERS_URL);
    res.retain(|x| *x != self_url);
    res
}

pub fn read_nodes() -> HashMap<String, Node> {
    let s1 = sled_db::get_or_empty(DB_NODES);
    let res1: HashMap<String, Node> = HashMap::new();
    let res2: Result<HashMap<String, Node>, serde_json::Error> = serde_json::from_str(&s1);
    if let Ok(h) = res2 {
        return h;
    }
    res1
}

pub fn update_nodes(n: Node) -> bool {
    let mut nodes = read_nodes();
    nodes.insert(n.id.clone(), n);
    let txt = serde_json::to_string(&nodes);
    if let Ok(s) = txt {
        sled_db::insert(DB_NODES, &s);
        return true;
    }
    false
}
// 当选举结束，leader就开始响应客户端的请求。流程如下：
// leader将客户端的请求命令作为一条新的条目写入日志。
// leader发送AppendEntries RPCs给其他的服务器去备份该日志条目。
// follower收到leader的AppendEntries RPCs，将该条目记录到日志并返回回应给leader。
// 当该条日志被安全的备份即leader收到了半数以上的机器回应该条目已记录，则可认为该条目是有效的，leader节点将该条目交由state machine处理，并将执行结果返回给客户端。leader节点在下次心跳等AppendEntries RPCs中，会记录可被提交的日志条目编号commitIndex。
// follower在收到leader的AppendEntries RPCs，其中会包含commitIndex，follower判断该条目是否已执行，若未执行则执行commitIndex以及之前的相应条目。

// raft中的snapshot机制如下
// 在Raft中集群中的每个节点都会自主的产生snapshot
// snapshot中包含当前state machine状态、last included index（snapshot中包含的最后一条日志条目的索引）、last included term（snapshot中包含的最后一条日志条目所属的term）、集群成员信息。
// 当leader发现其要发送给follower的日志条目已经在snapshot中，则会发送installSnapshot RPCs给follower，该种情况通常发生在集群有新节点加入的时候。
// follower收到InstallSnapshot RPC的时候将snapshot写入到本地，并根据snapshot内容重设state machine，如果本地有更老的snapshot或者日志，则可以丢弃。
