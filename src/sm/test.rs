use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::sm::newrsa::rsa_load_http_keys;
use crate::sm::sm_impl_methods::*;
use crate::sm::sm_impl_trait::*;
use std::collections::HashMap;

pub fn test() -> Result<(), MyError> {
    let txt = "fasdf".to_string();
    let sm = select();
    // sm.init()?;
    let hash = sm.hash(txt.clone());
    let sig = sm.sig(txt.clone())?;
    let ver = sm.sig_verify(txt.clone(), sig.clone());
    dbg!(hash, sig, ver);

    // 私钥http不变
    let (rsa_pk, rsa_sk) = rsa_load_http_keys()?;
    // 随时变的公钥加密
    let k = rand_16_u8().to_vec();
    let _iv = rand_16_u8().to_vec();
    let sym_hash = SymKey { key: k, iv: _iv };

    let asym_str = serde_json::to_string(&sym_hash)?;
    let asym_enc = sm.asym_enc(asym_str.clone(), rsa_pk)?;
    let asym_dec = sm.asym_dec(asym_enc.clone(), rsa_sk)?;
    dbg!(asym_str.clone(), asym_dec.clone());

    let body = sm.sym_enc(txt.clone(), asym_dec.clone())?;
    let decbody = sm.sym_dec(body.clone(), asym_str.clone())?;
    dbg!(txt, body, decbody);

    Ok(())
}
