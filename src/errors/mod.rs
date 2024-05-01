use std::fmt;

#[derive(Debug)]
pub struct ErrorBase {
    details: String,
}

impl ErrorBase {
    pub fn new(msg: &str) -> ErrorBase {
        ErrorBase { details: msg.to_string() }
    }

    pub fn new_box(msg: &str) -> Box<ErrorBase> {
        Box::new(ErrorBase::new(msg))
    }
}

impl fmt::Display for ErrorBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ErrorBase {
    fn description(&self) -> &str {
        &self.details
    }
}
