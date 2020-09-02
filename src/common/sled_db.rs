use crate::common::myerror::{MyError, StrError};
use crate::common::tools::get_dot_env;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
lazy_static! {
    static ref DB: sled::Db = {
        let path = get_dot_env("SLEDDIR");
        let _db = sled::open(path).unwrap();
        _db
    };
}

pub fn insert(k: &str, v: &str) -> bool {
    let kk = k.as_bytes().to_vec();
    let vv = v.as_bytes().to_vec();

    let res = DB.insert(kk, vv);
    if let Ok(_s) = res {
        return true;
    }
    false
}

pub fn _get(k: &str) -> Result<String, MyError> {
    let res = DB.get(&k);
    if let Ok(res1) = res {
        if let Some(res2) = res1 {
            let mut res3 = vec![];
            for i in res2.iter() {
                res3.push(*i);
            }
            let res4 = String::from_utf8(res3)?;
            return Ok(res4);
        }
    }
    Err(MyError::OtherError(StrError::new("get fail")))
}

pub fn get_or_empty(k: &str) -> String {
    let res = _get(k);
    if let Ok(s) = res {
        return s;
    }
    "".to_owned()
}
pub fn read_set_from_db(dbk: &str) -> HashSet<String> {
    let peers = {
        let a = get_or_empty(dbk);
        let res1: Result<HashSet<String>, serde_json::Error> = serde_json::from_str(&a);
        if let Ok(res2) = res1 {
            res2
        } else {
            let res3: HashSet<String> = HashSet::new();
            res3
        }
    };
    peers
}

pub fn update_set_from_db_and_str(dbk: &str, content: &str) -> bool {
    let mut peers = read_set_from_db(dbk);
    let c = content.trim().to_string();
    if c != "" {
        peers.insert(c);
    }
    let l_str = serde_json::to_string(&peers);
    if let Ok(s) = l_str {
        return insert(dbk, &s);
    }
    false
}

pub fn update_set_from_db_and_set(dbk: &str, l: HashSet<String>) -> bool {
    let mut peers = read_set_from_db(dbk);
    for content in l {
        let c = content.trim().to_string();
        if c != "" {
            peers.insert(c);
        }
    }
    let l_str = serde_json::to_string(&peers);
    if let Ok(s) = l_str {
        return insert(dbk, &s);
    }
    false
}

pub fn del_str_from_set(dbk: &str, e: &str) -> bool {
    let mut l = read_set_from_db(dbk);
    l.retain(|x| x != e);
    let js = serde_json::to_string(&l);
    if let Ok(s) = js {
        let res = insert(dbk, &s);
        return res;
    }
    false
}

pub fn must_del_str_from_set(dbk: &str, e: &str) {
    let mut l = read_set_from_db(dbk);
    l.retain(|x| x != e);
    let js = serde_json::to_string(&l);
    if let Ok(s) = js {
        let res = insert(dbk, &s);
        if !res {
            panic!("can not del {}: {}", dbk, e);
        }
    } else {
        panic!("can not del {}: {}", dbk, e);
    }
}
pub fn must_update_set_from_db_and_str(dbk: &str, content: &str) {
    let mut peers = read_set_from_db(dbk);
    let c = content.trim().to_string();
    if c != "" {
        peers.insert(c);
    }
    let l_str = serde_json::to_string(&peers);
    if let Ok(s) = l_str {
        let res = insert(dbk, &s);
        if !res {
            panic!("can not update {}: {}", dbk, content);
        }
    } else {
        panic!("can not update {}: {}", dbk, content);
    }
}
