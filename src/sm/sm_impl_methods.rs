use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::sm::newrsa::*;
use crate::sm::sm_impl_trait::Enctrait;
use cryptape_sm as libsm;
use libsm::sm2::signature::{Pubkey, Seckey, SigCtx, Signature};
use libsm::sm3::hash::Sm3Hash;
use libsm::sm4::{Cipher, Mode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static SM_ENC_NAME: &'static str = "sig";
#[derive(Debug, Copy, Clone)]
pub struct Sm {}

impl Enctrait for Sm {
    fn init(&self) -> Result<String, MyError> {
        let res1 = sm_save_sig_keys()?;
        Ok(res1)
    }
    fn hash(&self, content: String) -> String {
        sm3_hash(content)
    }

    fn sig(&self, content: String) -> Result<String, MyError> {
        let (pk, sk) = sm_load_sig_keys()?;
        let res = sm2_sign(content, &pk, &sk)?;
        Ok(res)
    }

    fn sig_verify(&self, origin: String, sig: String) -> bool {
        if let Ok((pk, _)) = sm_load_sig_keys() {
            return sm2_verify(origin, sig, &pk);
        }
        false
    }

    fn sym_enc(&self, content: String, keystr: String) -> Result<String, MyError> {
        let keys: SymKey = serde_json::from_str(&keystr)?;
        let res = sm4_enc(content, &keys.key, &keys.iv);
        Ok(res)
    }

    fn sym_dec(&self, content: String, keystr: String) -> Result<String, MyError> {
        let keys: SymKey = serde_json::from_str(&keystr)?;
        let res = sm4_dec(content, &keys.key, &keys.iv)?;
        Ok(res)
    }
    // 加密的内容是对称加密的iv和key Symkey ,json形式入
    fn asym_enc(&self, content: String, pubkey: String) -> Result<String, MyError> {
        let pk: HashMap<String, i64> = serde_json::from_str(&pubkey)?;
        let n = pk.get("n").ok_or("no n iv")?;
        let e = pk.get("e").ok_or("no  e key")?;
        let symkey = content.into_bytes();
        // 加密出来是i64，然后js打包
        let res1 = rsa_enc(*n, *e, symkey);
        let res2 = serde_json::to_string(&res1)?;
        Ok(res2)
    }
    fn asym_dec(&self, content: String, sk: String) -> Result<String, MyError> {
        let sk: HashMap<String, i64> = serde_json::from_str(&sk)?;
        let n = sk.get("n").ok_or("no iv")?;
        let d = sk.get("d").ok_or("no d key")?;
        // 加密出来是i64，然后js打包
        let enc: Vec<i64> = serde_json::from_str(&content)?;
        let res1 = rsa_dec(*n, *d, enc)?;
        let res2 = String::from_utf8(res1)?;
        Ok(res2)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymKey {
    pub key: Vec<u8>,
    pub iv: Vec<u8>,
}

pub fn sm2_create_pair() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    // 33,65
    let pk_raw = ctx.serialize_pubkey(&pk, true);
    // 32
    let sk_raw = ctx.serialize_seckey(&sk);
    (pk_raw, sk_raw)
}

//公私钥签名，公钥验证
pub fn sm2_sign(string: String, pk: &[u8], sk: &[u8]) -> Result<String, MyError> {
    let msg = string.as_bytes();
    let ctx = SigCtx::new();
    if let Ok(pk1) = ctx.load_pubkey(pk) {
        if let Ok(sk1) = ctx.load_seckey(sk) {
            let signature = ctx.sign(msg, &sk1, &pk1);
            let sig_u8 = signature.der_encode();
            let res = base64::encode(&sig_u8);
            return Ok(res);
        }
    }
    Err(MyError::OtherError(StrError::new("sing fail")))
}

// sig is base64 sig string
pub fn sm2_verify(origin: String, sig: String, pk: &[u8]) -> bool {
    let sig_u8 = base64::decode(&sig).unwrap();
    let ctx = SigCtx::new();
    let msg = origin.into_bytes();
    let mut res = false;

    if let Ok(pk1) = ctx.load_pubkey(pk) {
        if let Ok(sig_return) = Signature::der_decode(&sig_u8) {
            res = ctx.verify(&msg, &pk1, &sig_return);
        }
    }
    res
}

pub fn sm4_enc(content: String, rand_key: &[u8], rand_iv: &[u8]) -> String {
    let cipher = Cipher::new(&rand_key, Mode::Cfb);
    let cipher_text: Vec<u8> = cipher.encrypt(content.as_bytes(), &rand_iv);
    let res = base64::encode(&cipher_text);
    res
}

// content is base64 enc
pub fn sm4_dec(content: String, rand_key: &[u8], rand_iv: &[u8]) -> Result<String, MyError> {
    let cipher = Cipher::new(&rand_key, Mode::Cfb);
    let cipher_text: Vec<u8> = base64::decode(&content).expect("base 64 err");
    let new_text: Vec<u8> = cipher.decrypt(&cipher_text[..], &rand_iv);
    let res = String::from_utf8(new_text)?;
    Ok(res)
}

pub fn sm3_hash(content: String) -> String {
    let mut hash = Sm3Hash::new(content.as_bytes());
    let digest: [u8; 32] = hash.get_hash();
    let res = base64::encode(&digest);
    res
}

pub fn sm_save_sig_keys() -> Result<String, MyError> {
    let root = get_dot_env("CRTDIR");
    let pk_path = format!("{}/{}_pk", root, SM_ENC_NAME);
    let sk_path = format!("{}/{}_sk", root, SM_ENC_NAME);
    let (pk_raw, sk_raw) = sm2_create_pair();
    let pk_str = base64::encode(&pk_raw);
    let sk_str = base64::encode(&sk_raw);
    write_file_as_txt(&pk_path, pk_str);
    write_file_as_txt(&sk_path, sk_str);
    Ok("success".to_string())
}

pub fn sm_load_sig_keys() -> Result<(Vec<u8>, Vec<u8>), MyError> {
    let root = get_dot_env("CRTDIR");
    let pk_path = format!("{}/{}_pk", root, SM_ENC_NAME);
    let sk_path = format!("{}/{}_sk", root, SM_ENC_NAME);
    let pk_base64 = read_file_as_txt(&pk_path);
    let sk_base64 = read_file_as_txt(&sk_path);
    let pk = &base64::decode(&pk_base64).unwrap();
    let sk = &base64::decode(&sk_base64).unwrap();
    Ok((pk.to_vec(), sk.to_vec()))
}
