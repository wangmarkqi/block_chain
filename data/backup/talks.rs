use crate::common::myerror::{MyError, StrError};
use crate::common::tools::*;
use crate::msg::req::{post_json, MSG};
use crate::raft::config::*;
use crate::raft::db;
use crate::raft::node::Node;
use std::collections::{HashMap, HashSet};
use uuid::Version::Mac;
// talks:从env文件出发，不断更新累加数据库peers——url，找到领导的url结束。由于接受的一方会增加发送的url列表和发送方本身，所以，url会传播到网络.talks初始化了role的数据库值follower

#[derive(Serialize, Deserialize, Debug)]
// 主要目的：不断积累url，找领导，找到领导了，就闭嘴
pub struct Talks {
    pub id: String,
    pub which: String,
    pub url: String,
    // following 3 update in db
    pub peers_url: HashSet<String>,
    pub leader_uuid: String,
    pub leader_url: String,
    // update when elcetion propose
    // pub self_check: bool,
}

impl Sender<Talks> for Talks {
    fn send(&self, url: &str) -> Result<Talks, MyError> {
        let js = serde_json::to_string(&self)?;
        let m = MSG {
            which: self.which.clone(),
            body: js,
        };
        // 不发给自己，因为peer url包括了自己的
        if url == self.url {
            return Err(MyError::OtherError(StrError::new("can not send to myself")));
        }
        let res1 = post_json(url, m)?;
        let res2 = res1.body;
        if res1.which == SEND_ERR {
            return Err(MyError::OtherError(StrError::new(&res2)));
        }
        let res3: Talks = serde_json::from_str(&res2)?;
        Ok(res3)
    }
}

impl Replier for Talks {
    fn reply(self, which: &str) -> Result<MSG, MyError> {
        let mut res = Talks::new_talk()?;
        res.peers_url = update_set_from_db_and_set(DB_PEERS_URL, self.peers_url)?;
        let res3 = serde_json::to_string(&res)?;
        let res4 = MSG {
            which: which.to_owned(),
            body: res3,
        };
        Ok(res4)
    }
}

//  role: ROLE_FOLLOWER.to_owned(),
// 一开始出了个evn读出friend，然后发送takls，不断更新数据库的peers——url，发送接受方不断累加url，发送方找到leader url，不在talking
impl Talks {
    pub fn thread_talking_until_leader_come() -> Result<(), MyError> {
        // 初始化角色，leader
        db::insert(DB_LEADER_UUID, "");
        db::insert(DB_LEADER_URL, "");
        db::insert(DB_SELF_ROLE, ROLE_FOLLOWER);

        let mut ts = Talks::new_talk()?;
        'outer: loop {
            let self_role = db::get(DB_SELF_ROLE)?;
            if self_role != ROLE_FOLLOWER {
                break;
            };
            let peers_url = read_vec_from_db(DB_PEERS_URL)?;
            for url in peers_url {
                let res = ts
                    .send(&url)
                    .and_then(|feedback| Talks::update_talks(feedback));
                // let feedback = ts.send(&url)?;
                // let ts = Talks::update_talks(feedback)?;
                match res {
                    Ok(ts) => {
                        if ts.leader_uuid != "" {
                            break 'outer;
                        }
                    }
                    Err(e) => continue,
                }
            }
        }
        Ok(())
    }

    pub fn update_talks(t: Talks) -> Result<Talks, MyError> {
        // 如果url不等于自己，不等于空，就加入列表
        // 数据库更新了peer url

        let uuid = Node::self_uuid()?;
        let _url = Node::self_url()?;

        let h = update_set_from_db_and_set(DB_PEERS_URL, t.peers_url)?;

        let leader_from_peer = t.leader_uuid;
        let leader_url_from_peer = t.leader_url;
        let mut res = Talks {
            id: uuid,
            url: _url,
            which: MSG_TALKS.to_owned(),
            peers_url: h,
            leader_uuid: "".to_string(),
            leader_url: "".to_string(),
        };
        if leader_from_peer == "".to_string() {
            return Ok(res);
        }
        // 如果有领导id就加入数据库,这个步骤是talks核心目的
        db::insert(DB_LEADER_UUID, &leader_from_peer);
        db::insert(DB_LEADER_URL, &leader_url_from_peer);
        res.leader_uuid = leader_from_peer;
        res.leader_url = leader_url_from_peer;
        Ok(res)
    }
    // 最开始，角色follower，领导url“”
    pub fn new_talk() -> Result<Talks, MyError> {
        let uuid = Node::self_uuid()?;
        let _url = Node::self_url()?;

        let mut l = HashSet::new();
        // 把自己的url加入并更新数据库
        let _friends_url = get_dot_env("FRIENDS")?;
        let friends_url: Vec<String> = _friends_url.split(",").map(|s| s.to_string()).collect();
        for i in friends_url {
            l.insert(i);
        }
        let h = update_set_from_db_and_set(DB_PEERS_URL, l)?;
        let lid = read_str_from_db_or_empty(DB_LEADER_UUID);
        let lurl = read_str_from_db_or_empty(DB_LEADER_URL);
        Ok(Talks {
            id: uuid,
            url: _url,
            which: MSG_TALKS.to_owned(),
            peers_url: h,
            leader_uuid: lid,
            leader_url: lurl,
        })
    }
}
