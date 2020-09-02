use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use redis::Commands;

pub fn setmsg(key: &str, txt: &str) -> Result<String, MyError> {
    let a = _setmsg(key, txt);
    match a {
        Ok(_b) => return Ok("redis set success".to_owned()),
        Err(e) => {
            let c = e.to_string();
            return Err(MyError::OtherError(StrError::new(&c)));
        }
    }
}
pub fn _setmsg(key: &str, txt: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set(key, txt)?;
    Ok(())
}

pub fn getmsg(key: &str) -> Result<String, MyError> {
    let res = redis::Client::open("redis://127.0.0.1/")
        .and_then(|cli| cli.get_connection())
        .and_then(|mut con| con.get(key));
    if let Ok(a) = res {
        return Ok(a);
    }
    Err(MyError::OtherError(StrError::new("redis get fail")))
}

pub fn submsg(channel: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe(channel)?;
    let one_seconds = std::time::Duration::new(5, 0);
    pubsub.set_read_timeout(Some(one_seconds));

    loop {
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}

pub fn pubmsg(channel: &str, txt: &str) -> Result<String, MyError> {
    let res = redis::Client::open("redis://127.0.0.1/")
        .and_then(|cli| cli.get_connection())
        .and_then(|mut con| con.publish(channel, txt));
    if let Ok(()) = res {
        return Ok("redis pub success".to_string());
    }
    Err(MyError::OtherError(StrError::new("redis pub fail")))
}

pub fn test() {
    std::thread::spawn(|| submsg("frompy").unwrap());
    for _i in 1..10 {
        pubmsg("rust", "asdfa");
        std::thread::sleep(std::time::Duration::new(2, 2));
    }
}
