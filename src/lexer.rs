use crate::error::RutenError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //  -- literals --
    Number(f64),
    String(String),
    Identifier(String),
    True,
    False,
    None,

    // -- keywords --
    Import,
    Def,
    Return,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,

    // -- operators --
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,

    // -- delimiters --
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Newline,

    Eof,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, RutenError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // -- whitespace -- | note: except newlines
            ' ' | '\t' | '\r' => {
                chars.next();
            }
            // newlines
            '\n' => {
                chars.next();
                tokens.push(Token::Newline);
            }
            // -- comments -- | note: lowercase as per spec
            '#' => {
                chars.next();
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            // -- numbers --
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>()
                    .map_err(|_| RutenError::SyntaxError(format!("invalid number: {}", num_str)))?;
                tokens.push(Token::Number(num));
            }
            // -- strings --
            '"' | '\'' => {
                let quote = ch;
                chars.next();
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    chars.next();
                    if ch == quote {
                        break;
                    }
                    if ch == '\\' {
                        if let Some(&next_ch) = chars.peek() {
                            chars.next();
                            match next_ch {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '"' => string.push('"'),
                                '\'' => string.push('\''),
                                _ => {
                                    string.push('\\');
                                    string.push(next_ch);
                                }
                            }
                        }
                    } else {
                        string.push(ch);
                    }
                }
                tokens.push(Token::String(string));
            }
            // -- identifiers and keywords --
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token = match ident.as_str() {
                    "import" => Token::Import,
                    "def" => Token::Def,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "in" => Token::In,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "True" => Token::True,
                    "False" => Token::False,
                    "None" => Token::None,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "not" => Token::Not,
                    _ => Token::Identifier(ident),
                };
                tokens.push(token);
            }
            // -- operators and delimiters --
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '%' => {
                chars.next();
                tokens.push(Token::Percent);
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    return Err(RutenError::SyntaxError("unexpected character '!'".to_string()));
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            '[' => {
                chars.next();
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RightBracket);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RightBrace);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '.' => {
                chars.next();
                tokens.push(Token::Dot);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            _ => {
                return Err(RutenError::SyntaxError(format!("unexpected character: '{}'", ch)));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}
