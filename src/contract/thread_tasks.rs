use super::dispatch::dispatch_contract;
use crate::common::sled_db;
use crate::common::sled_db::read_set_from_db;
use crate::common::tools::{get_dot_env, CONTRACT_FAIL, CONTRACT_PROGRESS, CONTRACT_SUCCESS};
use crate::contract::contracts::SmartContract;
use crate::raft::config::ConfigRaft;

pub fn thread_tasks() {
    // 初始化角色，leader
    init_args();

    let ticks = ConfigRaft::new().heartbeat_tick as u64;
    let d = std::time::Duration::new(ticks, 0);
    loop {
        std::thread::sleep(d);
        let ids = read_set_from_db(CONTRACT_PROGRESS);
        sled_db::insert(CONTRACT_PROGRESS, "");
        if ids.len() == 0 {
            continue;
        }

        for id in ids {
            if let Ok(task) = SmartContract::get(&id) {
                let res = dispatch_contract(task);
                dbg!(res);
            } else {
                dbg!("no tasks");
            }
        }
    }
}

// 最开始，角色follower，领导url“”
fn init_args() {
    let iswin = cfg!(target_os = "windows");
    if iswin {
        let key = "PYTHONHOME";
        let py = get_dot_env(key);
        dbg!(py);
    }
    sled_db::insert(CONTRACT_SUCCESS, "");
    sled_db::insert(CONTRACT_FAIL, "");
    sled_db::insert(CONTRACT_PROGRESS, "");
}
pub fn test() {
    let key = "PYTHONHOME";
    let py = get_dot_env(key);
    dbg!(py);
}
