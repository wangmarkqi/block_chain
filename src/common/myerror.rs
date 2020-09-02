use crate::common::myerror::MyError::Rocket;
use std::error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
    VarError(std::env::VarError),
    Utf8Error(std::string::FromUtf8Error),
    Json(serde_json::Error),
    Rocket(rocket::config::ConfigError),
    OtherError(StrError),
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::IoError(ref err) => write!(f, "IO error: {}", err),
            MyError::Rocket(ref err) => write!(f, "rocket error: {}", err),
            MyError::ParseError(ref err) => write!(f, "Parse error: {}", err),
            MyError::VarError(ref err) => write!(f, "Var error: {}", err),
            MyError::Utf8Error(ref err) => write!(f, "Utf8 error: {}", err),
            MyError::Json(ref err) => write!(f, "Utf8 error: {}", err),
            MyError::OtherError(ref err) => write!(f, "Other error: {}", err),
        }
    }
}

impl error::Error for MyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MyError::IoError(ref err) => Some(err),
            MyError::Rocket(ref err) => Some(err),
            MyError::ParseError(ref err) => Some(err),
            MyError::VarError(ref err) => Some(err),
            MyError::Utf8Error(ref err) => Some(err),
            MyError::Json(ref err) => Some(err),
            MyError::OtherError(ref err) => Some(err),
        }
    }
}
impl From<&str> for MyError {
    fn from(error: &str) -> Self {
        MyError::OtherError(StrError::new(error))
    }
}

impl From<rocket::config::ConfigError> for MyError {
    fn from(error: rocket::config::ConfigError) -> Self {
        MyError::Rocket(error)
    }
}
impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::Json(error)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MyError::Utf8Error(error)
    }
}

impl From<StrError> for MyError {
    fn from(error: StrError) -> Self {
        MyError::OtherError(error)
    }
}

impl From<std::env::VarError> for MyError {
    fn from(error: std::env::VarError) -> Self {
        MyError::VarError(error)
    }
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error)
    }
}

impl From<num::ParseIntError> for MyError {
    fn from(error: num::ParseIntError) -> Self {
        MyError::ParseError(error)
    }
}

// 创造自己的string error 和上面配合使用
#[derive(Debug)]
pub struct StrError {
    details: String,
}

impl StrError {
    pub fn new(msg: &str) -> StrError {
        StrError {
            details: msg.to_string(),
        }
    }
}

impl Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for StrError {
    fn description(&self) -> &str {
        &self.details
    }
}
// impl Debug for StrError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.details)
//     }
// }

// impl Debug for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match *self {
//             MyError::IoError(ref err) => write!(f, "IO error: {}", err),
//             MyError::ParseError(ref err) => write!(f, "Parse error: {}", err),
//             MyError::VarError(ref err) => write!(f, "Var error: {}", err),
//             MyError::Utf8Error(ref err) => write!(f, "Utf8 error: {}", err),
//             MyError::Json(ref err) => write!(f, "Utf8 error: {}", err),
//             MyError::OtherError(ref err) => write!(f, "Other error: {}", err),
//         }
//     }
// }
//
