use crate::common::myerror::MyError;
use crate::common::tools::get_dot_env;
use crate::common::tools::*;
use crate::contract::dispatch;
use crate::trans::dispatch_tls::dispatch_tls;
use crate::trans::inner_sm_server::start_sm_inner;
use crate::trans::req_tls::MSG;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/inner", format = "json", data = "<m>")]
fn inner(m: Json<MSG>) -> Json<MSG> {
    let which = &m.which;
    let body = &m.body;
    let msg = dispatch_tls(which, body);
    let m = {
        match msg {
            Ok(m) => m,
            Err(e) => MSG {
                which: SEND_ERR.to_owned(),
                body: e.to_string(),
            },
        }
    };
    Json(m)
}

pub fn start_tls_inner() -> Result<(), MyError> {
    let h = get_dot_env("INNER");
    let v: Vec<&str> = h.split(":").collect();
    let host = v[0].to_owned();
    let p = v[1].to_owned();

    let port = p.parse::<u16>()?;
    let key = get_dot_env("TLSKEY");
    let crt = get_dot_env("TLSCERT");
    let config = Config::build(Environment::Staging)
        .address(host)
        .port(port)
        .tls(crt, key)
        .finalize()?;

    rocket::custom(config)
        .mount("/", routes![inner, index])
        .launch();

    Ok(())
    // rocket::ignite().mount("/", routes![inner, index]).launch();
}
