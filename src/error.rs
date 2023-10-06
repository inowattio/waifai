
pub type WFResult<T> = Result<T, WFError>;

enum WFError {
    CommandIO,
}
