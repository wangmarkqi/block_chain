use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::sled_db::del_str_from_set;
use crate::common::tools::*;
use crate::contract::callpy::*;
use crate::raft::ask;
use crate::raft::config::*;
use crate::raft::node::Node;
use crate::sm::sm_impl_methods::*;
use crate::sm::sm_impl_trait::*;
use crate::trans::req_tls::{select_inner_trans, MSG};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Version::Mac;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SmartContract {
    pub contract: String,
    // url为空表示执行自己
    #[serde(default = "default_resource")]
    pub url: String,
    #[serde(default = "default_bool_false")]
    pub hash: bool,
    #[serde(default = "default_bool_false")]
    pub sign: bool,
    #[serde(default = "default_bool_false")]
    pub after: bool,
    #[serde(default = "default_bool_false")]
    pub rollback: bool,
    // 如果是直接存证，数据放到args
    #[serde(default = "default_resource")]
    pub args: String,
    #[serde(default = "default_resource")]
    pub term_res: String,
    #[serde(default = "default_resource")]
    pub after_res: String,
    #[serde(default = "default_resource")]
    pub rollback_res: String,
    #[serde(default = "default_vec_string")]
    pub signatures: Vec<String>,
    #[serde(default = "default_resource")]
    // 主要是存证后把整个区块放进去，返回给审计模块
    pub saved_block: String,
    #[serde(default = "default_resource")]
    pub voucher: String,
    #[serde(default = "default_resource")]
    pub status: String,
}

impl SmartContract {
    pub fn must_save(&self) {
        let txt = serde_json::to_string(self);
        if let Ok(s) = txt {
            let res = sled_db::insert(&self.voucher, &s);
            if res {
                return;
            }
        }
        panic!("save contract fail");
    }

    pub fn get(uuid: &str) -> Result<SmartContract, MyError> {
        let txt = sled_db::get_or_empty(uuid);
        let n: SmartContract = serde_json::from_str(&txt)?;
        Ok(n)
    }
    pub fn enqueue(&self) {
        sled_db::must_update_set_from_db_and_str(CONTRACT_PROGRESS, &self.voucher);
        self.must_save();
    }
    pub fn queue2success(&mut self) {
        sled_db::must_update_set_from_db_and_str(CONTRACT_SUCCESS, &self.voucher);
        self.status = SUCCESS.to_string();
        self.must_save();
    }
    pub fn queue2fail(&mut self) {
        sled_db::must_update_set_from_db_and_str(CONTRACT_FAIL, &self.voucher);
        self.must_save();
    }
    pub fn smart_contract_self_terms(&mut self) -> SmartContract {
        let terms_res = call_terms(&self.contract, &self.args);
        if let Err(e) = terms_res {
            // 移动到失败队列，添加失败原因到res，保存key uuid
            self.status = format!("自身智能合约terms执行失败：{}", e);
            self.queue2fail();
            self.must_save();
            return self.clone();
        };
        let self_terms_res = terms_res.unwrap();
        self.term_res = self_terms_res;
        self.status = SUCCESS.to_string();
        self.queue2success();
        self.must_save();
        self.clone()
    }
}
