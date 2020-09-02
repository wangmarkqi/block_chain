use super::req_ca::CA;
use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::raft::ask::Snapshot;
use crate::raft::config::Replier;
use crate::trans::req_tls::MSG;
use serde::{Deserialize, Serialize};
use serde_json::ser::CharEscape::Tab;
use std::collections::HashMap;

pub fn dispatch_tls(which: &str, body: &str) -> Result<MSG, MyError> {
    match which {
        ASK => {
            let a: Snapshot = serde_json::from_str(body)?;
            a.reply()
        }
        _ => Err(MyError::OtherError(StrError::new("which not match"))),
    }
}
