use super::sled_db;
use crate::common::myerror::{MyError, StrError};
use chrono::prelude::*;
use chrono::Utc;
use rand::Rng;
use reqwest::blocking::ClientBuilder;
use reqwest::Error;
use std::ffi::OsString;
use std::fs::{read_to_string, File};
use std::io::prelude::*;

pub const RPC: &'static str = "/rpc/";
pub const SEND_ERR: &'static str = "error";
pub const ANSWER: &'static str = "answer";
pub const YES: &'static str = "yes";
pub const NO: &'static str = "no";
pub const SUCCESS: &'static str = "success";
pub const FAIL: &'static str = "fail";

pub const DB_PEERS_URL: &'static str = "peers_url";
pub const DB_LEADER_URL: &'static str = "leader_url";
pub const DB_SELF_ROLE: &'static str = "self_role";
pub const DB_LATEST_HEART_BEAT: &'static str = "latest_heat_beat";
pub const DB_NODES: &'static str = "nodes";

pub const ROLE_LEADER: &'static str = "leader";
pub const ROLE_CANDIDATE: &'static str = "candidate";
pub const ROLE_FOLLOWER: &'static str = "follower";

pub const ASK: &'static str = "ask";
pub const ASK_SNAPSHOT: &'static str = "ask_snapshot";
// 领导对群众
pub const ASK_SAVE_BLOCK: &'static str = "ask_save_block";
pub const ASK_REQ_VOTE: &'static str = "ask_req_vote";
pub const ASK_SMART_CONTRACT: &'static str = "ask_smart_contract";
// 群众对领导，但是只能加最后一个
pub const ASK_APPEND_ENTRY: &'static str = "ask_append_entry";

pub const RPC_PEERS_URL: &'static str = "rpc_peers_url";
pub const RPC_SELF_CHECK: &'static str = "rpc_self_check";
pub const RPC_LATEST_HEARTBEAT: &'static str = "rpc_latest_heartbeat";
pub const RPC_LEADER_URL: &'static str = "rpc_leader_url";
pub const RPC_MAX_BLOCK: &'static str = "rpc_max_block";
pub const RPC_SELF_ROLE: &'static str = "rpc_self_role";
pub const RPC_BLOCK_BY_HEIGHT: &'static str = "rpc_block_by_height";
pub const RPC_APPEND_ENTRYT: &'static str = "append_entry";
pub const RPC_SMART_CONTRACT: &'static str = "smart_contract";
pub const RPC_CONTRACT_STATUS: &'static str = "contract_status";
pub const RPC_ALL_BLOCKS: &'static str = "all_blocks";

pub const CA_ENC: &'static str = "ca_enc";
pub const CA_DEC: &'static str = "ca_dec";
pub const CA_PK: &'static str = "ca_pk";
pub const CA_ENC_KEY: &'static str = "ca_enc_key";
pub const CA_ENC_BODY: &'static str = "ca_enc_body";
pub const CA_URL_SUFIX: &'static str = "ca";

pub const CONTRACT_PROGRESS: &'static str = "contract_progress";
pub const CONTRACT_SUCCESS: &'static str = "contract_success";
pub const CONTRACT_FAIL: &'static str = "contract_fail";

pub const FEEDBACK_IN_PROGRESS: &'static str = "交易正在处理中,请等待";
pub const FEEDBACK_IN_SUCCESS: &'static str = "交易成功";
pub const FEEDBACK_IN_FAIL: &'static str = "交易失败";

pub fn get_dot_env(name: &str) -> String {
    dotenv::dotenv().ok();
    if let Ok(v) = std::env::var(name) {
        return v;
    }
    panic!("!!!!!!!!!!no env var: {}", name);
}

pub fn rand_16_u8() -> [u8; 16] {
    let mut v: [u8; 16] = [0; 16];
    for x in v.iter_mut() {
        *x = rand::random()
    }
    v
}

pub fn write_file_as_txt(path: &str, content: String) -> bool {
    let res = _write_file_as_txt(path, content);
    match res {
        Ok(_a) => true,
        Err(_e) => false,
    }
}
pub fn _write_file_as_txt(path: &str, content: String) -> Result<String, MyError> {
    let mut output: File = File::create(path)?;
    write!(output, "{}", content)?;
    Ok("ok".to_string())
}

pub fn read_file_as_txt(file: &str) -> String {
    let res = read_to_string(file);
    if let Ok(s) = res {
        return s;
    }
    "".to_owned()
}

pub fn time_now_str() -> String {
    let local = Local::now();
    let s = local.to_rfc3339();
    s
}

pub fn time_differ(origin: &str) -> i64 {
    let a = Local::now().time();
    let b = DateTime::parse_from_rfc3339(origin).unwrap();
    let c = b.time();
    let differ = a - c;
    println!("Total time taken to run is {}", differ.num_seconds());
    let res = differ.num_seconds();
    res
}

pub fn post_str(url: &str, content: &str) -> String {
    let res = ClientBuilder::new()
        .build()
        .and_then(|c| c.post(url).body(content.to_owned()).send());
    match res {
        Ok(resp) => {
            let res2 = resp.text().unwrap();
            return res2;
        }
        Err(e) => {
            let s = e.to_string();
            let er = MyError::OtherError(StrError::new(&s));
            dbg!(er);
            return "".to_string();
        }
    }
}
pub fn get_uuid() -> String {
    let my_uuid = uuid::Uuid::new_v4();
    let res = format!("{}", my_uuid);
    res
}
pub fn default_resource() -> String {
    "".to_string()
}
pub fn default_bool_false() -> bool {
    false
}
pub fn default_u64() -> u64 {
    0
}
pub fn default_vec_string() -> Vec<String> {
    vec![]
}
pub fn test() {
    let host = get_dot_env("INNER");
    let v: Vec<&str> = host.split(":").collect();
    let host = v[0];
    let ip = v[1];
}

pub fn start_ca_golang() -> Result<(), MyError> {
    let root = get_dot_env("ROOT");
    let ca_exe = format!("{}/ca/ca.exe", root);
    let ca_sh = format!("{}/ca/ca", root);
    dbg!((&ca_exe, &ca_sh));
    let iswin = cfg!(target_os = "windows");
    dbg!(iswin);
    let mut child = if iswin {
        std::process::Command::new("cmd")
            .arg("/c")
            .arg(&ca_exe)
            .spawn()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&ca_sh)
            .spawn()
            .expect("failed to execute process")
    };
    Ok(())
}
pub fn create_dir_if_not_exists(dir: &str) {
    let p = std::path::Path::new(dir);
    if p.exists() {
        return;
    };
    std::fs::create_dir(p).unwrap();
}
