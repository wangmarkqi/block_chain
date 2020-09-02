use super::dispatch_sm::reply_ca_pk;
use super::req_sm::post_json_origine;
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::raft::config::Sender;
use crate::sm::sm_impl_methods::SymKey;
use crate::sm::sm_impl_trait;
use crate::sm::sm_impl_trait::Enctrait;
use crate::trans::req_tls::MSG;
use reqwest::blocking::ClientBuilder;
use reqwest::Error;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CA {
    #[serde(default = "default_resource")]
    pub which: String,
    #[serde(default = "default_resource")]
    pub caname: String,
    #[serde(default = "default_resource")]
    pub caurl: String,
    #[serde(default = "default_resource")]
    pub org: String,
    #[serde(default = "default_resource")]
    pub sym_key: String,
    #[serde(default = "default_resource")]
    pub pk: String,
    #[serde(default = "default_resource")]
    pub err: String,
}

pub fn default_resource() -> String {
    "".to_string()
}

impl CA {
    pub fn post_json(&self, url: &str) -> Result<CA, MyError> {
        let body = json!(self);
        let res = ClientBuilder::new()
            .build()
            .and_then(|c| c.post(url).json(&body).send());
        match res {
            Ok(resp) => {
                let res2 = resp.text().unwrap();
                let res3 = serde_json::from_str(&res2)?;
                return Ok(res3);
            }
            Err(e) => {
                let s = e.to_string();
                let er = MyError::OtherError(StrError::new(&s));
                return Err(er);
            }
        }
    }
}

pub fn ca_url() -> String {
    let caurl = get_dot_env("CAURL");
    let url = format!("http://{}/{}", caurl, CA_URL_SUFIX);
    url
}

pub fn ask_peer_pb(url: &str) -> Result<CA, MyError> {
    // 客户端
    // 请求1首先问对方的公钥
    let m = MSG {
        which: CA_PK.to_owned(),
        body: "".to_owned(),
    };
    let res1 = post_json_origine(url, m)?;
    let res3: CA = serde_json::from_str(&res1.body)?;
    Ok(res3)
}

pub fn ask_ca_enc_keys(mut ca: CA) -> Result<Vec<String>, MyError> {
    // 生成对称加密秘钥
    let mut v = vec![];
    let k = rand_16_u8().to_vec();
    let i = rand_16_u8().to_vec();

    let symk = SymKey { iv: i, key: k };
    let orig_key_str = serde_json::to_string(&symk)?;
    ca.sym_key = orig_key_str.clone();

    // 请求对方2然后请求自己的ca把iv k用对方的公钥加密，保存原文，发送iv key,密文
    let caurl = ca_url();
    let res4 = ca.post_json(&caurl)?;

    // 都是b64，前面是没有加密，后面是加密,加密传过去，没有加密留下来解密
    v.push(orig_key_str);
    v.push(res4.sym_key);
    Ok(v)
}

// server端口用到这个技术
pub fn ask_ca_dec_keys(enckey: &str) -> Result<String, MyError> {
    let mut ca = CA {
        which: CA_DEC.to_owned(),
        caurl: "".to_owned(),
        caname: "".to_owned(),
        err: "".to_owned(),
        org: "".to_owned(),
        pk: "".to_owned(),
        sym_key: "".to_owned(),
    };
    ca.sym_key = enckey.to_owned();

    let caurl = ca_url();
    let res = ca.post_json(&caurl)?;
    let symkey = res.sym_key;
    Ok(symkey)
}

pub fn test() {
    let res1 = reply_ca_pk().unwrap();
    let res2: CA = serde_json::from_str(&res1.body).unwrap();
    let res3 = ask_ca_enc_keys(res2).unwrap();
    let enc = &res3[1];
    let res4 = ask_ca_dec_keys(&enc).unwrap();
    dbg!(res4.clone());
    let sm = sm_impl_trait::select();
    let res5 = sm.sym_enc("asfaf".to_string(), res4.clone()).unwrap();
    let res6 = sm.sym_dec(res5, res4).unwrap();
    dbg!(res6);
    // ask_ca_dec_keys("asdas").unwrap();
}
