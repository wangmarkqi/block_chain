use super::req_sm::SmTran;
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use reqwest::blocking::ClientBuilder;
use reqwest::Error;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MSG {
    pub which: String,
    pub body: String,
}

pub trait InnerTrans {
    fn post_json(&self, url: &str, msg: MSG) -> Result<MSG, MyError>;
}

pub fn select_inner_trans() -> Box<dyn InnerTrans> {
    let method = get_dot_env("INNERTRANS");

    if method == "tls" {
        return Box::new(Tls {});
    };
    if method == "sm" {
        return Box::new(SmTran {});
    }
    Box::new(Tls {})
}

pub struct Tls {}

impl InnerTrans for Tls {
    fn post_json(&self, url: &str, msg: MSG) -> Result<MSG, MyError> {
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
}
