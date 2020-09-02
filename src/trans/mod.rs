pub mod dispatch_sm;
pub mod dispatch_tls;
pub mod inner_sm_server;
pub mod inner_tls_server;
pub mod req_ca;
pub mod req_sm;
pub mod req_tls;
pub mod test;

use crate::common::myerror::*;
use crate::common::tools::*;
pub fn start_inner() -> Result<(), MyError> {
    let method = get_dot_env("INNERTRANS");

    if method == "tls" {
        inner_tls_server::start_tls_inner()?;
    } else {
        inner_sm_server::start_sm_inner()?;
    };
    Ok(())
}
