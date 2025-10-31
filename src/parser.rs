use crate::error::RutenError;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    None,
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Member {
        object: Box<Expr>,
        member: String,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Import(String),
    Assign {
        name: String,
        value: Expr,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Expression(Expr),
}

pub type Program = Vec<Stmt>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn skip_newlines(&mut self) {
        while self.match_token(&[Token::Newline]) {}
    }

    fn parse_program(&mut self) -> Result<Program, RutenError> {
        let mut statements = Vec::new();
        self.skip_newlines();
        
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }
        
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, RutenError> {
        self.skip_newlines();
        
        if self.match_token(&[Token::Import]) {
            return self.parse_import();
        }
        if self.match_token(&[Token::Def]) {
            return self.parse_function_def();
        }
        if self.match_token(&[Token::Return]) {
            return self.parse_return();
        }
        if self.match_token(&[Token::If]) {
            return self.parse_if();
        }
        if self.match_token(&[Token::While]) {
            return self.parse_while();
        }
        if self.match_token(&[Token::For]) {
            return self.parse_for();
        }
        if self.match_token(&[Token::Break]) {
            self.skip_newlines();
            return Ok(Stmt::Break);
        }
        if self.match_token(&[Token::Continue]) {
            self.skip_newlines();
            return Ok(Stmt::Continue);
        }

        // assignment or expression
        let expr = self.parse_expression()?;
        
        if let Expr::Identifier(name) = &expr {
            if self.match_token(&[Token::Equal]) {
                let value = self.parse_expression()?;
                self.skip_newlines();
                return Ok(Stmt::Assign {
                    name: name.clone(),
                    value,
                });
            }
        }
        
        self.skip_newlines();
        Ok(Stmt::Expression(expr))
    }

    fn parse_import(&mut self) -> Result<Stmt, RutenError> {
        let module = if let Token::Identifier(module) = self.advance().clone() {
            module
        } else {
            return Err(RutenError::SyntaxError("expected module name after 'import'".to_string()));
        };
        
        self.skip_newlines();
        Ok(Stmt::Import(module))
    }

    fn parse_function_def(&mut self) -> Result<Stmt, RutenError> {
        let name = if let Token::Identifier(n) = self.advance() {
            n.clone()
        } else {
            return Err(RutenError::SyntaxError("expected function name".to_string()));
        };

        if !self.match_token(&[Token::LeftParen]) {
            return Err(RutenError::SyntaxError("expected '(' after function name".to_string()));
        }

        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                if let Token::Identifier(param) = self.advance() {
                    params.push(param.clone());
                } else {
                    return Err(RutenError::SyntaxError("expected parameter name".to_string()));
                }
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }

        if !self.match_token(&[Token::RightParen]) {
            return Err(RutenError::SyntaxError("expected ')' after parameters".to_string()));
        }

        if !self.match_token(&[Token::Colon]) {
            return Err(RutenError::SyntaxError("expected ':' after function signature".to_string()));
        }

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Stmt::FunctionDef { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, RutenError> {
        if self.check(&Token::Newline) || self.is_at_end() {
            self.skip_newlines();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expression()?;
            self.skip_newlines();
            Ok(Stmt::Return(Some(expr)))
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, RutenError> {
        let condition = self.parse_expression()?;
        
        if !self.match_token(&[Token::Colon]) {
            return Err(RutenError::SyntaxError("expected ':' after if condition".to_string()));
        }

        self.skip_newlines();
        let then_branch = self.parse_block()?;

        let else_branch = if self.match_token(&[Token::Else]) {
            if !self.match_token(&[Token::Colon]) {
                return Err(RutenError::SyntaxError("expected ':' after else".to_string()));
            }
            self.skip_newlines();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, RutenError> {
        let condition = self.parse_expression()?;
        
        if !self.match_token(&[Token::Colon]) {
            return Err(RutenError::SyntaxError("expected ':' after while condition".to_string()));
        }

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, RutenError> {
        let var = if let Token::Identifier(v) = self.advance() {
            v.clone()
        } else {
            return Err(RutenError::SyntaxError("expected variable name in for loop".to_string()));
        };

        if !self.match_token(&[Token::In]) {
            return Err(RutenError::SyntaxError("expected 'in' in for loop".to_string()));
        }

        let iterable = self.parse_expression()?;

        if !self.match_token(&[Token::Colon]) {
            return Err(RutenError::SyntaxError("expected ':' after for clause".to_string()));
        }

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Stmt::For { var, iterable, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, RutenError> {
        let mut statements = Vec::new();
        
        // simple block parsing - collect statements until we hit a dedent or keyword
        while !self.is_at_end() 
            && !self.check(&Token::Else) 
            && !self.check(&Token::Def)
            && !self.check(&Token::If)
            && !self.check(&Token::While)
            && !self.check(&Token::For) {
            
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.parse_statement()?);
            
            // break on double newline or specific tokens
            if statements.len() > 0 && (self.check(&Token::Else) || self.is_at_end()) {
                break;
            }
        }
        
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Expr, RutenError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_and()?;

        while self.match_token(&[Token::Or]) {
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&[Token::And]) {
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(&[Token::EqualEqual, Token::NotEqual]) {
            let op = match self.tokens[self.current - 1] {
                Token::EqualEqual => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_term()?;

        while self.match_token(&[Token::Less, Token::LessEqual, Token::Greater, Token::GreaterEqual]) {
            let op = match self.tokens[self.current - 1] {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_factor()?;

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match self.tokens[self.current - 1] {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_unary()?;

        while self.match_token(&[Token::Star, Token::Slash, Token::Percent]) {
            let op = match self.tokens[self.current - 1] {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, RutenError> {
        if self.match_token(&[Token::Minus, Token::Not]) {
            let op = match self.tokens[self.current - 1] {
                Token::Minus => UnaryOp::Neg,
                Token::Not => UnaryOp::Not,
                _ => unreachable!(),
            };
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, RutenError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&[Token::LeftParen]) {
                // function call
                let mut args = Vec::new();
                if !self.check(&Token::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(&[Token::Comma]) {
                            break;
                        }
                    }
                }
                if !self.match_token(&[Token::RightParen]) {
                    return Err(RutenError::SyntaxError("expected ')' after arguments".to_string()));
                }
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.match_token(&[Token::LeftBracket]) {
                // indexing
                let index = self.parse_expression()?;
                if !self.match_token(&[Token::RightBracket]) {
                    return Err(RutenError::SyntaxError("expected ']' after index".to_string()));
                }
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&[Token::Dot]) {
                // member access
                if let Token::Identifier(member) = self.advance() {
                    expr = Expr::Member {
                        object: Box::new(expr),
                        member: member.clone(),
                    };
                } else {
                    return Err(RutenError::SyntaxError("expected member name after '.'".to_string()));
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, RutenError> {
        if let Token::Number(n) = self.peek() {
            let num = *n;
            self.advance();
            return Ok(Expr::Number(num));
        }

        if let Token::String(s) = self.peek() {
            let string = s.clone();
            self.advance();
            return Ok(Expr::String(string));
        }

        if self.match_token(&[Token::True]) {
            return Ok(Expr::Bool(true));
        }

        if self.match_token(&[Token::False]) {
            return Ok(Expr::Bool(false));
        }

        if self.match_token(&[Token::None]) {
            return Ok(Expr::None);
        }

        if let Token::Identifier(name) = self.peek() {
            let ident = name.clone();
            self.advance();
            return Ok(Expr::Identifier(ident));
        }

        if self.match_token(&[Token::LeftParen]) {
            let expr = self.parse_expression()?;
            if !self.match_token(&[Token::RightParen]) {
                return Err(RutenError::SyntaxError("expected ')' after expression".to_string()));
            }
            return Ok(expr);
        }

        if self.match_token(&[Token::LeftBracket]) {
            let mut elements = Vec::new();
            if !self.check(&Token::RightBracket) {
                loop {
                    elements.push(self.parse_expression()?);
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            if !self.match_token(&[Token::RightBracket]) {
                return Err(RutenError::SyntaxError("expected ']' after list elements".to_string()));
            }
            return Ok(Expr::List(elements));
        }

        if self.match_token(&[Token::LeftBrace]) {
            let mut pairs = Vec::new();
            if !self.check(&Token::RightBrace) {
                loop {
                    let key = self.parse_expression()?;
                    if !self.match_token(&[Token::Colon]) {
                        return Err(RutenError::SyntaxError("expected ':' in dictionary".to_string()));
                    }
                    let value = self.parse_expression()?;
                    pairs.push((key, value));
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            if !self.match_token(&[Token::RightBrace]) {
                return Err(RutenError::SyntaxError("expected '}' after dictionary".to_string()));
            }
            return Ok(Expr::Dict(pairs));
        }

        Err(RutenError::SyntaxError(format!("unexpected token: {:?}", self.peek())))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, RutenError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}