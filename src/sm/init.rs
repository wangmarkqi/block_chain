use crate::common::myerror::MyError;
use crate::common::tools::*;
use crate::sm::sm_impl_trait::select;
// 注意impl trait 一定要写下面的话，不然无法确定类型
use crate::sm::sm_impl_trait::Enctrait;
pub fn start_app() -> Result<(), MyError> {
    let sm = select();
    sm.init()?;
    let my_uuid = uuid::Uuid::new_v4();
    let u = my_uuid.to_string();
    let p = get_dot_env("CRTDIR");
    let f = format!("{}/uuid", p);
    write_file_as_txt(&f, u);
    Ok(())
}
