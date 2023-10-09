
pub type WFResult<T> = Result<T, WFError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WFError {
    CommandIO,
    CommandParse,
    CommandErr(String),
    WifiAction(String),
    HotspotCreate(String),
}
