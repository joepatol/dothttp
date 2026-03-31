#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownMethod { method: String, line: usize },
    InvalidRequestLine { line: usize, content: String },
    InvalidHeader { line: usize, content: String },
    InvalidVariableDeclaration { line: usize, content: String },
    UndefinedVariable { name: String, line: usize },
    UnexpectedEndOfInput,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownMethod { method, line } => {
                write!(f, "Parse error on line {line}: unknown HTTP method '{method}'")
            }
            ParseError::InvalidRequestLine { line, content } => {
                write!(f, "Parse error on line {line}: invalid request line '{content}'")
            }
            ParseError::InvalidHeader { line, content } => {
                write!(f, "Parse error on line {line}: invalid header line '{content}'")
            }
            ParseError::InvalidVariableDeclaration { line, content } => {
                write!(
                    f,
                    "Parse error on line {line}: invalid variable declaration '{content}'"
                )
            }
            ParseError::UndefinedVariable { name, line } => {
                write!(f, "Parse error on line {line}: undefined variable '{name}'")
            }
            ParseError::UnexpectedEndOfInput => {
                write!(f, "Parse error: unexpected end of input")
            }
        }
    }
}

impl std::error::Error for ParseError {}
