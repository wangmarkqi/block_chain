use crate::common::myerror::MyError;
use crate::common::tools::get_dot_env;
use crate::common::tools::*;
use crate::contract::contracts::SmartContract;
use crate::contract::dispatch;
use crate::contract::query::answer;
use crate::trans::dispatch_tls::dispatch_tls;
use crate::trans::req_tls::MSG;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::content;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/query/<api>/<body>")]
fn query(api: &RawStr, body: &RawStr) -> content::Json<String> {
    let res = answer(api, body);
    match res {
        Ok(s) => content::Json(s),
        Err(e) => content::Json(e.to_string()),
    }
    // format!("Hello, {}!", name.as_str())
}

#[post("/transaction", format = "json", data = "<t>")]
fn trans(t: Json<SmartContract>) -> Json<SmartContract> {
    let tt = &*t;
    let res = dispatch::add_voucher_and_enqueue(tt.clone());
    Json(res)
}

pub fn start_rpc() -> Result<(), MyError> {
    let h = get_dot_env("RPC");
    let v: Vec<&str> = h.split(":").collect();
    let host = v[0].to_owned();
    let p = v[1].to_owned();

    let port = p.parse::<u16>()?;
    let config = Config::build(Environment::Staging)
        .address(host)
        .port(port)
        .finalize()?;

    rocket::custom(config)
        .mount("/", routes![query, trans, index])
        .launch();

    Ok(())
}
