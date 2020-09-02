#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;

use crate::common::tools::*;
use std::process::Command;

pub mod blocks;
pub mod common;
pub mod contract;
pub mod raft;
pub mod sm;
pub mod trans;

fn main() {
    std::thread::spawn(|| raft::run::run_thread());
    std::thread::spawn(|| trans::start_inner().unwrap());
    std::thread::spawn(|| contract::thread_tasks::thread_tasks());
    contract::rpc_server::start_rpc().unwrap();
    // blocks::test::test();
}

// target = "x86_64-unknown-linux-musl"
