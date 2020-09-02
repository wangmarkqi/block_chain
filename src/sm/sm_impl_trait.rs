use crate::common::myerror::MyError;
use crate::common::tools::get_dot_env;
use crate::sm::sm_impl_methods::Sm;
pub trait Enctrait {
    fn init(&self) -> Result<String, MyError>;
    fn hash(&self, content: String) -> String;
    fn sig(&self, content: String) -> Result<String, MyError>;
    fn sig_verify(&self, origin: String, sig: String) -> bool;
    fn sym_enc(&self, content: String, key: String) -> Result<String, MyError>;
    fn sym_dec(&self, content: String, key: String) -> Result<String, MyError>;
    fn asym_enc(&self, content: String, pubkey: String) -> Result<String, MyError>;
    fn asym_dec(&self, content: String, sk: String) -> Result<String, MyError>;
}

// pub fn select() -> impl Enctrait {
pub fn select() -> Box<dyn Enctrait> {
    let method = get_dot_env("ECNMETHOD");

    let res = match method.as_str() {
        "sm" => Box::new(Sm {}),
        _ => Box::new(Sm {}),
    };
    res
}
