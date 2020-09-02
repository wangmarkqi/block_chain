use crate::blocks::block::Block;
/// A `Chain` representation.
use crate::blocks::save;
use crate::blocks::save::Savetrait;
use crate::common::myerror::MyError;
use crate::sm::sm_impl_trait;
use crate::sm::sm_impl_trait::Enctrait;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    pub ommit: HashSet<u64>,
    pub wrong: HashSet<u64>,
    // pub more: HashSet<u64>,
    pub height: u64,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            ommit: HashSet::new(),
            wrong: HashSet::new(),
            // more: HashSet::new(),
            height: 0,
        }
    }
    pub fn first_ommit_wrong() -> u64{
        let sa = save::select();
        let all = sa.all();
        let mut ommit = HashSet::new();
        for (i, v) in all.iter().enumerate() {
            if i > 0 {
                let differ = v - all[i - 1];
                let b = sa.query(*v);
            let trust = b.is_trusty_for();
                if !trust{
                    return i as u64 -1;
                }
                if differ > 1 {
                    return i as u64 -1;
                }
            }
        }
        let l=all.len() as u64
        return l-1;
    }

    pub fn real_height() -> bool {
         let sa = save::select();
        let all = sa.all();
        let l1=all.len() as u64;
        let l2=Chain::first_ommit_wrong();

        // let has_more = self.find_more(leader)?;
        self.find_height();
        if has_ommit || has_wrong {
            return true;
        }
        false
    }
}
