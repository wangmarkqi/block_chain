use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::contract::callpy::*;
use crate::contract::contracts::SmartContract;
use crate::raft::config::*;
use crate::raft::node::Node;
use crate::sm::sm_impl_trait;
use crate::trans::req_tls::InnerTrans;
use crate::trans::req_tls::{select_inner_trans, MSG};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Version::Mac;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub which: String,
    pub node: Node,
    pub peers_url: HashSet<String>,
    pub value: String,
    pub leader_url: String,
}

impl Sender<MSG> for Snapshot {
    fn send(&self, url: &str) -> Result<MSG, MyError> {
        let js = serde_json::to_string(&self)?;
        let m = MSG {
            which: ASK.to_owned(),
            body: js,
        };
        // 不发给自己，因为peer url包括了自己的
        if url == self.node.url {
            return Err(MyError::OtherError(StrError::new("can not send to myself")));
        }

        // 在这个地方实现了sm加密传输和tls加密传输的选择和转化
        let trans_method = get_dot_env("INNERTRANS");
        let new_url = {
            if trans_method == "tls" {
                format!("https://{}/inner", url)
            } else {
                format!("http://{}/inner", url)
            }
        };
        let tran = select_inner_trans();
        let res1 = tran.post_json(&new_url, m)?;
        let body = &res1.body;
        let which = &res1.which;
        if which == SEND_ERR {
            return Err(MyError::OtherError(StrError::new(&body)));
        }
        Ok(res1)
    }
}

impl Replier for Snapshot {
    fn reply(&self) -> Result<MSG, MyError> {
        let src = self.clone();
        let from_role = src.node.role.clone();
        let from_url = src.node.url;
        let mut from_peers = src.peers_url;
        from_peers.insert(from_url);
        if from_role == ROLE_LEADER {
            let js = serde_json::to_string(&from_peers)?;
            sled_db::insert(DB_PEERS_URL, &js);
        } else {
            sled_db::update_set_from_db_and_set(DB_PEERS_URL, from_peers);
        }

        match self.which.as_str() {
            ASK_SNAPSHOT => self.answer_snapshot(),
            ASK_SAVE_BLOCK => self.answer_save_block(),
            ASK_APPEND_ENTRY => self.answer_append_entry(),
            ASK_REQ_VOTE => self.answer_req_vote(),
            ASK_SMART_CONTRACT => self.answer_smart_contract(),
            _ => Err(MyError::OtherError(StrError::new("no matched ask"))),
        }
    }
}

// 作为回答问题一方的代码，问问题的以及对答案的处理散落在各处
impl Snapshot {
    pub fn new_ask(a: &str) -> Snapshot {
        let n = Node::new_from_block();
        let peers = read_peers_url_without_self();
        let leader = sled_db::get_or_empty(DB_LEADER_URL);
        Snapshot {
            // 两层路由，msg是ask，ask是细节
            which: a.to_owned(),
            node: n,
            peers_url: peers,
            value: "".to_owned(),
            leader_url: leader,
        }
    }

    pub fn pack_answer(&self, s: &str) -> MSG {
        MSG {
            which: ANSWER.to_owned(),
            body: s.to_string(),
        }
    }
    // include heart beat ,update peers,find leader
    pub fn answer_snapshot(&self) -> Result<MSG, MyError> {
        let which = &self.which;
        // snap来源于leader heart beat
        if &self.node.role == ROLE_LEADER {
            let now = time_now_str();
            let role = sled_db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);
            let hb = sled_db::insert(DB_LATEST_HEART_BEAT, &now);
            let leader_url = sled_db::insert(DB_LEADER_URL, &self.node.url);
            // 来源于群众，说明是领导hb，save to hashmap nodes
        }
        let mut snap = Snapshot::new_ask(which);
        // if self is leader
        snap.value = YES.to_owned();
        let s = serde_json::to_string(&snap)?;
        let res = self.pack_answer(&s);
        Ok(res)
    }

    // 接到心跳，保存领导node，回答yes,更新时间，多次没有yes，领导删掉你,只要接到心跳，就把自己变成群众

    // 领导才能干
    //领导发群众
    pub fn answer_save_block(&self) -> Result<MSG, MyError> {
        // 把value放到 block的value
        // save block first if block is wrong del all after
        let _b = &self.value;
        let b = _b.clone();
        let bs: Vec<Block> = serde_json::from_str(&b)?;
        for b in bs {
            b.add();
        }
        let a = self.pack_answer(YES);
        return Ok(a);
    }

    //群众发给领导,纯的什么返回什么msg body==block
    pub fn answer_append_entry(&self) -> Result<MSG, MyError> {
        let ro = sled_db::get_or_empty(DB_SELF_ROLE);
        let content = &self.value;
        // 不是领导存vec
        let mut msg = self.pack_answer("");
        if ro == ROLE_LEADER {
            let b = Block::new(content.clone());
            let success = b.add();
            if success {
                let txt = serde_json::to_string(&b)?;
                msg.body = txt;
            } else {
                msg.body = FAIL.to_owned();
            }
        } else {
            msg.body = FAIL.to_owned();
        }
        Ok(msg)
    }

    pub fn answer_req_vote(&self) -> Result<MSG, MyError> {
        // 从ask中查到id号
        let height_from_out = self.node.block.id;
        let b = Block::max();
        let height_from_self = b.id;
        let leader_url = sled_db::get_or_empty(DB_LEADER_URL);
        let mut answer = self.pack_answer("");
        if leader_url != "" {
            answer.body = NO.to_owned();
        } else if height_from_self > height_from_out {
            answer.body = NO.to_string();
        } else {
            answer.body = YES.to_string();
        };
        return Ok(answer);
    }

    fn answer_smart_contract(&self) -> Result<MSG, MyError> {
        let mut answer = self.pack_answer("");
        let body = &self.value;
        let mut contract: SmartContract = serde_json::from_str(body)?;
        let name = contract.contract.clone();
        let arg = contract.args.clone();
        if contract.rollback {
            call_rollback(&name)?;
            contract.rollback_res = YES.to_string();
            let txt = serde_json::to_string(&contract)?;
            answer.body = txt;
            return Ok(answer);
        }

        let sm = sm_impl_trait::select();
        if contract.sign {
            let sigtxt = sm.sig(contract.voucher.clone()).expect("sign fail");
            contract.signatures.push(sigtxt);
        }

        let out_terms_res = contract.term_res;
        let myterms_res = call_terms(&name, &arg)?;
        contract.term_res = myterms_res;

        // 在要求after以及结果一致性的情况下
        if out_terms_res == contract.term_res && contract.after {
            call_after(&name)?;
            contract.after_res = YES.to_string();
        } else {
            contract.after_res = NO.to_string();
        }
        let txt = serde_json::to_string(&contract)?;

        answer.body = txt;
        Ok(answer)
    }
}
