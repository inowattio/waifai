
pub type WFResult<T> = Result<T, WFError>;

#[derive(Debug)]
pub enum WFError {
    CommandIO,
    CommandParse,
    HotspotCreate(String),
}
