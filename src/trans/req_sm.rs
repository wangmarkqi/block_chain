use super::req_ca::{ask_ca_enc_keys, ask_peer_pb};
use super::req_tls::{InnerTrans, MSG};
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::sm::sm_impl_trait;
use crate::sm::sm_impl_trait::Enctrait;
use reqwest::blocking::ClientBuilder;
use reqwest::Error;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
pub struct SmTran {}

// 客户端
// 请求ca1首先问对方的公钥
// 请求对方2然后请求自己的ca把iv k用对方的公钥加密，保存原文，发送iv key密文
// 服务端
// 请求ca1 把把非对称iv，key通过自己的ca用自己私钥解开
// 应答客户请求2 trans body对称sm4加密，hash 对称密文 发过去，
//
// 客户端
// 解密

// 请求1对方，然后对称加密解开msg，重新组装
impl InnerTrans for SmTran {
    fn post_json(&self, url: &str, msg: MSG) -> Result<MSG, MyError> {
        let ca = ask_peer_pb(url)?;
        let l = ask_ca_enc_keys(ca)?;
        let orgingkey_str = l[0].clone();
        let content = msg.body;
        let sm = sm_impl_trait::select();
        let enc = sm.sym_enc(content, orgingkey_str.clone())?;
        let mut dic = HashMap::new();
        dic.insert(CA_ENC_KEY.to_owned(), l[1].clone());
        dic.insert(CA_ENC_BODY.to_owned(), enc);
        let b = serde_json::to_string(&dic)?;
        let newmsg = MSG {
            which: msg.which,
            body: b,
        };
        let res1 = post_json_origine(url, newmsg)?;
        let decbody = sm.sym_dec(res1.body, orgingkey_str.clone())?;
        let res2 = MSG {
            which: res1.which,
            body: decbody,
        };
        Ok(res2)
    }
}
pub fn post_json_origine(url: &str, msg: MSG) -> Result<MSG, MyError> {
    let body = json!(msg);
    let res = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .and_then(|c| c.post(url).json(&body).send());
    match res {
        Ok(resp) => {
            let res2 = resp.text().unwrap();
            let res3: MSG = serde_json::from_str(&res2)?;
            return Ok(res3);
        }
        Err(e) => {
            let s = e.to_string();
            let er = MyError::OtherError(StrError::new(&s));
            return Err(er);
        }
    }
}
pub fn test() {
    let t = get_dot_env("FRIENDS");
    let url = format!("http://{}/inner", t);
    let smt = SmTran {};
    let msg = MSG {
        which: "ask".to_string(),
        body: "asdfa".to_owned(),
    };
    let res = smt.post_json(&url, msg);
    dbg!(res);
}
