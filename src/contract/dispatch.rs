use super::contracts::SmartContract;
use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::contract::callpy::*;
use crate::raft::ask;
use crate::raft::config::*;
use crate::raft::node::Node;
use crate::sm::sm_impl_methods::*;
use crate::sm::sm_impl_trait::*;
use crate::trans::req_tls::{select_inner_trans, MSG};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Version::Mac;

// 上来就入列，分配voucher，分为智能合约和存证
pub fn add_voucher_and_enqueue(mut sm: SmartContract) -> SmartContract {
    // let mut sm: SmartContract = serde_json::from_str(t)?;
    if sm.voucher == "" {
        let uuid = get_uuid();
        sm.voucher = uuid.clone();
    }
    sm.term_res = "已经加入任务队列".to_string();
    sm.status = SUCCESS.to_string();
    sm.enqueue();
    sm
}

// api design here 一个存证，一个合约，如果不希望after把after设置false，校验后，再after  true
pub fn dispatch_contract(mut contract: SmartContract) -> SmartContract {
    let sm = select();
    // 对于存证和合约自己的签名都加了,注意存证只有一个签名
    if contract.sign {
        let sigtxt = sm.sig(contract.voucher.clone()).expect("sign fail");
        contract.signatures.push(sigtxt);
    }
    if contract.contract == RPC_APPEND_ENTRYT {
        append_entry(contract)
    } else {
        smart_contract(contract)
    }
}

fn smart_contract(mut cont: SmartContract) -> SmartContract {
    let mut contract = cont.smart_contract_self_terms();

    let mut a = ask::Snapshot::new_ask(ASK_SMART_CONTRACT);
    let txt = serde_json::to_string(&contract).unwrap();
    a.value = txt;
    let send_res = a.send(&contract.url);

    if let Err(e) = send_res {
        contract.status = format!("对方智能合约执行失败：{}", e);
        contract.queue2fail();
        return contract;
    };
    let msg = send_res.unwrap();
    let mut t: SmartContract = serde_json::from_str(&msg.body).unwrap();
    if t.after_res == YES {
        let myafter = call_after(&contract.contract);
        if let Err(e) = myafter {
            t.after_res = format!(
                "我方智能合约after执行失败：{},对方已成功，需要回滚操作,我方失败原因：",
                e
            );
            // 只有合约执行成功才可以上链条
            let res2 = roll_back_after(t);
            return res2;
        } else {
            t.after_res = "双方智能合约执行成功".to_string();
        }
    };
    t.queue2success();
    append_entry(t)
}

fn roll_back_after(mut contract: SmartContract) -> SmartContract {
    contract.rollback = true;
    let mut a = ask::Snapshot::new_ask(ASK_SMART_CONTRACT);
    let txt = serde_json::to_string(&contract).unwrap();
    a.value = txt;
    let send_res = a.send(&contract.url);
    if let Err(e) = send_res {
        contract.status = format!("对方智能合约回滚传输失败：{}", e);
        contract.queue2fail();
        return contract;
    };
    let msg = send_res.unwrap();
    let mut t: SmartContract = serde_json::from_str(&msg.body).unwrap();
    if t.rollback_res == YES {
        t.rollback_res = "对方智能合约回滚成功".to_string();
    } else {
        t.rollback_res = "对方方智能合约回滚执行失败".to_string();
    }
    t.queue2fail();
    t
}

// 就是保存contract args的内容
pub fn append_entry(mut contract: SmartContract) -> SmartContract {
    let sm = select();
    if contract.hash {
        contract.term_res = sm.hash(contract.term_res);
        contract.after_res = sm.hash(contract.after_res);
        contract.args = sm.hash(contract.args);
        contract.url = sm.hash(contract.url);
    }
    let txt = serde_json::to_string(&contract).expect("json fail");
    _append_entry(&txt, contract)
}

// body就是要加的明文,block value
fn _append_entry(body: &str, mut contract: SmartContract) -> SmartContract {
    let self_role = sled_db::get_or_empty(DB_SELF_ROLE);
    if self_role == ROLE_LEADER {
        let b = Block::new(body.to_owned());
        let success = b.add();
        if success {
            let saved = serde_json::to_string(&b).unwrap();
            contract.saved_block = saved;
            contract.queue2success();
        } else {
            contract.status = "本节点（领导）没有成功加入数据".to_owned();
            contract.queue2fail();
        }
        return contract;
    }
    let leader_url = sled_db::get_or_empty(DB_LEADER_URL);
    if leader_url == "" {
        contract.status = "找不到领导节点或者没有相应".to_owned();
        contract.queue2fail();
        return contract;
    }
    let mut a = ask::Snapshot::new_ask(ASK_APPEND_ENTRY);
    a.value = body.to_owned();
    let res = a.send(&leader_url);
    if let Ok(res) = res {
        if res.body == FAIL {
            contract.status = "领导节点无法成功加入数据".to_owned();
            contract.queue2fail();
            return contract;
        } else {
            contract.saved_block = res.body;
            contract.queue2success();
            return contract;
        }
    };
    contract.status = "没有到达领导节点".to_owned();
    contract.queue2fail();
    contract
}
