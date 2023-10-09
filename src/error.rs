
pub type WFResult<T> = Result<T, WFError>;

pub enum WFError {
    CommandIO,
    CommandParse,
    HotspotCreate,
}
