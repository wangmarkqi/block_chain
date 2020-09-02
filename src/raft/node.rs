use crate::blocks::block::Block;
use crate::blocks::save;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::config::*;
use crate::trans::req_tls::{select_inner_trans, MSG};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    pub url: String,
    pub role: String,
    // 以下是变量
    pub block: Block,
}

impl Node {
    pub fn self_uuid() -> String {
        let up = get_dot_env("CRTDIR");
        let uf = format!("{}/uuid", up);
        let uuid = read_file_as_txt(&uf);
        uuid
    }
    pub fn self_url() -> String {
        let url = get_dot_env("INNER");
        url
    }

    pub fn new_from_block() -> Node {
        let uuid = Node::self_uuid();
        let _url = Node::self_url();
        let _role = sled_db::get_or_empty(DB_SELF_ROLE);

        let lastb = Block::max();
        let n = Node {
            id: uuid,
            url: _url,
            role: _role,
            block: lastb,
        };
        n
    }
    pub fn save(&self) -> bool {
        let n = self;
        update_nodes(n.clone())
    }
    pub fn get_node(uuid: &str) -> Result<Node, MyError> {
        let txt = sled_db::get_or_empty(uuid);
        let n: Node = serde_json::from_str(&txt)?;
        Ok(n)
    }
}
