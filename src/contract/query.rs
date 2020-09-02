use crate::blocks::block::Block;
use crate::common::myerror::{MyError, StrError};
use crate::common::sled_db;
use crate::common::tools::*;
use crate::raft::config::Replier;
use crate::trans::req_tls::MSG;
use serde::{Deserialize, Serialize};
use serde_json::ser::CharEscape::Tab;

pub fn answer(api: &str, body: &str) -> Result<String, MyError> {
    match api {
        RPC_BLOCK_BY_HEIGHT => block_by_height(body),
        RPC_ALL_BLOCKS => all_blocks(),
        RPC_PEERS_URL => sled_db::_get(DB_PEERS_URL),
        RPC_LEADER_URL => sled_db::_get(DB_LEADER_URL),
        RPC_MAX_BLOCK => max_block(),
        RPC_SELF_ROLE => sled_db::_get(DB_SELF_ROLE),
        RPC_LATEST_HEARTBEAT => sled_db::_get(DB_LATEST_HEART_BEAT),
        RPC_CONTRACT_STATUS => sled_db::_get(body),
        _ => Err(MyError::OtherError(StrError::new("which not match"))),
    }
}
fn all_blocks() -> Result<String, MyError> {
    let mut l = vec![];
    let b = Block::max();
    for i in 1..b.id + 1 {
        let a = Block::get_block(i);
        l.push(a);
    }
    let s = serde_json::to_string(&l)?;
    Ok(s)
}
fn max_block() -> Result<String, MyError> {
    let b = Block::max();
    let s = serde_json::to_string(&b)?;
    Ok(s)
}
fn block_by_height(h: &str) -> Result<String, MyError> {
    let height = h.parse::<u64>()?;
    let b = Block::get_block(height);
    let s = serde_json::to_string(&b)?;
    Ok(s)
}
pub fn test() {
    all_blocks().unwrap();
}
