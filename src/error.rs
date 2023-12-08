use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub type WFResult<T> = Result<T, WFError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WFError {
    CommandIO,
    CommandParse,
    CommandErr(String),
    WifiAction(String),
    HotspotCreate(String),
}

impl Display for WFError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl Error for WFError {}
