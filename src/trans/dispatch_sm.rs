use super::req_ca::ask_ca_dec_keys;
use super::req_ca::CA;
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::raft::ask::Snapshot;
use crate::raft::config::Replier;
use crate::sm::sm_impl_trait;
use crate::sm::sm_impl_trait::Enctrait;
use crate::trans::req_tls::MSG;
use serde::{Deserialize, Serialize};
use serde_json::ser::CharEscape::Tab;
use std::collections::HashMap;

// msg的body--》snapshot
pub fn dispatch_sm(wh: &str, body: &str) -> Result<MSG, MyError> {
    match wh {
        ASK => {
            let dic: HashMap<String, String> = serde_json::from_str(body)?;
            let emp = "".to_owned();
            let encbody = dic.get(CA_ENC_BODY).clone().unwrap_or(&emp);
            let enckey = dic.get(CA_ENC_KEY).clone().unwrap_or(&emp);
            let symkey = ask_ca_dec_keys(enckey)?;
            let sm = sm_impl_trait::select();
            let newbody = sm.sym_dec(encbody.to_string(), symkey.clone())?;
            let a: Snapshot = serde_json::from_str(&newbody)?;
            let out = a.reply()?;
            let outbody = sm.sym_enc(out.body, symkey.clone())?;
            let newmsg = MSG {
                which: wh.to_owned(),
                body: outbody,
            };
            Ok(newmsg)
        }
        CA_PK => reply_ca_pk(),
        _ => Err(MyError::OtherError(StrError::new("which not match"))),
    }
}

pub fn reply_ca_pk() -> Result<MSG, MyError> {
    let cn = get_dot_env("CANAME");
    let cu = get_dot_env("CAURL");
    let og = get_dot_env("ORG");
    let pb = get_dot_env("CAPUB");
    let wh = CA_ENC.to_owned();
    let p = read_file_as_txt(&pb);
    let ca = CA {
        which: wh,
        caname: cn,
        caurl: cu,
        org: og,
        pk: p,
        err: "".to_owned(),
        sym_key: "".to_string(),
    };
    let b = serde_json::to_string(&ca)?;
    Ok(MSG {
        which: CA_PK.to_owned(),
        body: b,
    })
}
