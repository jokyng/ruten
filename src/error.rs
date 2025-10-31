use std::fmt;

#[derive(Debug, Clone)]
pub enum RutenError {
    SyntaxError(String),
    RuntimeError(String),
    TypeError(String),
    NameError(String),
    ImportError(String),
}

impl fmt::Display for RutenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RutenError::SyntaxError(msg) => write!(f, "syntax error: {}", msg),
            RutenError::RuntimeError(msg) => write!(f, "runtime error: {}", msg),
            RutenError::TypeError(msg) => write!(f, "type error: {}", msg),
            RutenError::NameError(msg) => write!(f, "name error: {}", msg),
            RutenError::ImportError(msg) => write!(f, "import error: {}", msg),
        }
    }
}

impl std::error::Error for RutenError {}