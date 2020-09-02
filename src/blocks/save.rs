use crate::blocks::block::Block;
use crate::common::myerror::MyError;
use crate::common::tools::*;
use std::result::Result;

pub trait Savetrait {
    fn add(&self, block: Block) -> bool;
    fn query(&self, id: u64) -> Block;
    fn all(&self) -> Vec<u64>;
}

pub fn select() -> Box<dyn Savetrait> {
    let method = get_dot_env("ECNMETHOD");

    let res = match method.as_str() {
        "txt" => Box::new(Txt {}),
        _ => Box::new(Txt {}),
    };
    res
}

pub struct Txt {}

impl Savetrait for Txt {
    fn add(&self, block: Block) -> bool {
        let root = get_dot_env("BLOCKDIR");
        let id = block.id;
        let path = format!("{}{}", root, id.to_string());
        let txt = serde_json::to_string(&block);
        if let Ok(t) = txt {
            let a = write_file_as_txt(&path, t);
            return a;
        }
        false
    }
    fn query(&self, id: u64) -> Block {
        let root = get_dot_env("BLOCKDIR");
        let path = format!("{}{}", root, id.to_string());
        let txt = read_file_as_txt(&path);
        let res: Result<Block, serde_json::Error> = serde_json::from_str(&txt);
        if let Ok(b) = res {
            return b;
        }
        Block::genesis()
    }

    fn all(&self) -> Vec<u64> {
        let res = _all();
        match res {
            Ok(v) => v,
            Err(_v) => vec![],
        }
    }
}

pub fn _all() -> Result<Vec<u64>, MyError> {
    let root = get_dot_env("BLOCKDIR");
    let mut entries = std::fs::read_dir(root)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    entries.sort();
    let mut v = vec![];
    for i in entries.iter() {
        let f = std::path::Path::new(&i);
        let m = f.clone().file_stem().unwrap();
        let p = m.to_str().unwrap().to_owned();
        if p.starts_with(".") {
            continue;
        };
        let p2 = p.parse::<u64>();
        match p2 {
            Ok(f) => v.push(f.clone()),
            Err(e) => println!("can not parse block file name to u64,block file e=={:?}", e),
        }
    }
    v.sort();
    Ok(v)
}
