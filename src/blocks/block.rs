use crate::blocks::save;
use crate::blocks::save::Savetrait;
use crate::common::myerror::MyError;
use crate::sm::sm_impl_trait;
use crate::sm::sm_impl_trait::Enctrait;
use chrono::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Block {
    pub id: u64,
    pub value: String,
    pub pre_id: u64,
    pub pre_hash: String,
    pub timestamp: DateTime<Local>,
}

impl Block {
    pub fn genesis() -> Block {
        // let local: DateTime<Local> = Local::now();
        let local: DateTime<Local> = Local.ymd(2020, 3, 19).and_hms(17, 00, 11);
        Block {
            id: 0,
            value: "这是创世区块".to_owned(),
            pre_id: 0,
            pre_hash: "上帝说：要有区块".to_owned(),
            timestamp: local,
        }
    }
    // 真正的最大区块，必须不漏不错
    pub fn max() -> Block {
        let id = get_real_block_max_index();
        let sa = save::select();
        let b = sa.query(id);
        b
    }

    pub fn new(v: String) -> Block {
        let local: DateTime<Local> = Local::now();
        let maxb = Block::max();
        let pre_str = serde_json::to_string(&maxb).unwrap();
        // 前一个全部+这个内容
        let hash_str = pre_str + &v;
        let enc = sm_impl_trait::select();

        let b = Block {
            id: maxb.id + 1,
            value: v,
            pre_id: maxb.id,
            pre_hash: enc.hash(hash_str),
            timestamp: local,
        };
        b
    }
    pub fn add(&self) -> bool {
        let sa = save::select();
        let res = sa.add(self.to_owned());
        res
    }
    pub fn get_block(id: u64) -> Block {
        let sa = save::select();
        let b = sa.query(id);
        b
    }
    pub fn is_trusty_for(&self) -> bool {
        let res = self._is_trusty_for();
        match res {
            Ok(b) => b,
            Err(_e) => false,
        }
    }
    pub fn _is_trusty_for(&self) -> Result<bool, MyError> {
        let id = self.id;
        if id == 0 {
            return Ok(true);
        }
        let sa = save::select();
        let enc = sm_impl_trait::select();
        let pre = sa.query(id - 1);
        let pre_str = serde_json::to_string(&pre)?;
        let prehash = enc.hash(pre_str + &self.value);
        let hasheqaull = prehash == self.pre_hash;
        let timeorder = pre.timestamp < self.timestamp;
        let res = hasheqaull && timeorder;
        Ok(res)
    }
}

fn get_real_block_max_index() -> u64 {
    let sa = save::select();
    let all = sa.all();
    if all.len() == 0 {
        let genisis = Block::genesis();
        sa.add(genisis);
        return 0;
    }
    let mut max_index = all.len() as u64 - 1;
    for (i, v) in all.iter().enumerate() {
        if i > 0 {
            let differ = v - all[i - 1];
            let b = sa.query(*v);
            let trust = b.is_trusty_for();
            if !trust {
                max_index = i as u64 - 1;
            }
            if differ > 1 {
                max_index = i as u64 - 1;
            }
        }
    }
    let id = all[max_index as usize];
    id
}
