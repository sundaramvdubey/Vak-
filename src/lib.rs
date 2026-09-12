use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}
impl Span {
    fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Int(String),
    Float(String),
    String(String),
    Fn,
    Let,
    Mut,
    Struct,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    True,
    False,
    Bool,
    I32,
    U32,
    U64,
    F64,
    StringType,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    Bang,
    Eq,
    Amp,
    Arrow,
    DotDot,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,
    Dot,
    Eof,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: Option<&'static str>,
    pub message: String,
    pub span: Span,
}
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                f,
                "error[{code}]: {} at {}:{}",
                self.message, self.span.line, self.span.column
            ),
            None => write!(
                f,
                "error: {} at {}:{}",
                self.message, self.span.line, self.span.column
            ),
        }
    }
}

fn keyword(s: &str) -> Option<TokenKind> {
    Some(match s {
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        "mut" => TokenKind::Mut,
        "struct" => TokenKind::Struct,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "Bool" => TokenKind::Bool,
        "I32" => TokenKind::I32,
        "U32" => TokenKind::U32,
        "U64" => TokenKind::U64,
        "F64" => TokenKind::F64,
        "String" => TokenKind::StringType,
        _ => return None,
    })
}

pub fn lex(source: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let bytes = source.as_bytes();
    let mut i = 0;
    let mut line = 1;
    let mut col = 1;
    let mut out = Vec::new();
    let mut errors = Vec::new();
    let advance = |i: &mut usize, line: &mut usize, col: &mut usize| {
        if bytes[*i] == b'\n' {
            *line += 1;
            *col = 1;
        } else {
            *col += 1;
        }
        *i += 1;
    };
    while i < bytes.len() {
        let start = i;
        let sl = line;
        let sc = col;
        let c = bytes[i] as char;
        if c.is_ascii_whitespace() {
            advance(&mut i, &mut line, &mut col);
            continue;
        }
        if c == '/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                advance(&mut i, &mut line, &mut col);
            }
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            advance(&mut i, &mut line, &mut col);
            while i < bytes.len()
                && ((bytes[i] as char).is_ascii_alphanumeric() || bytes[i] == b'_')
            {
                advance(&mut i, &mut line, &mut col);
            }
            let s = &source[start..i];
            out.push(Token {
                kind: keyword(s).unwrap_or_else(|| TokenKind::Ident(s.into())),
                span: Span::new(start, i, sl, sc),
            });
            continue;
        }
        if c.is_ascii_digit() {
            advance(&mut i, &mut line, &mut col);
            while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                advance(&mut i, &mut line, &mut col);
            }
            let mut float = false;
            if i < bytes.len()
                && bytes[i] == b'.'
                && i + 1 < bytes.len()
                && (bytes[i + 1] as char).is_ascii_digit()
            {
                float = true;
                advance(&mut i, &mut line, &mut col);
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    advance(&mut i, &mut line, &mut col);
                }
            }
            let s = &source[start..i];
            out.push(Token {
                kind: if float {
                    TokenKind::Float(s.into())
                } else {
                    TokenKind::Int(s.into())
                },
                span: Span::new(start, i, sl, sc),
            });
            continue;
        }
        if c == '"' {
            advance(&mut i, &mut line, &mut col);
            let mut value = String::new();
            let mut ok = false;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch == '"' {
                    advance(&mut i, &mut line, &mut col);
                    ok = true;
                    break;
                }
                if ch == '\\' {
                    advance(&mut i, &mut line, &mut col);
                    if i >= bytes.len() {
                        break;
                    }
                    let esc = bytes[i] as char;
                    value.push(match esc {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        _ => esc,
                    });
                    advance(&mut i, &mut line, &mut col);
                } else {
                    value.push(ch);
                    advance(&mut i, &mut line, &mut col);
                }
            }
            if ok {
                out.push(Token {
                    kind: TokenKind::String(value),
                    span: Span::new(start, i, sl, sc),
                });
            } else {
                errors.push(Diagnostic {
                    code: None,
                    message: "unterminated string literal".into(),
                    span: Span::new(start, i, sl, sc),
                });
            }
            continue;
        }
        let two = if i + 1 < bytes.len() {
            Some(&source[i..i + 2])
        } else {
            None
        };
        let kind = match two {
            Some("==") => Some((TokenKind::EqEq, 2)),
            Some("!=") => Some((TokenKind::NotEq, 2)),
            Some("<=") => Some((TokenKind::LtEq, 2)),
            Some(">=") => Some((TokenKind::GtEq, 2)),
            Some("&&") => Some((TokenKind::AndAnd, 2)),
            Some("||") => Some((TokenKind::OrOr, 2)),
            Some("->") => Some((TokenKind::Arrow, 2)),
            Some("..") => Some((TokenKind::DotDot, 2)),
            _ => None,
        };
        if let Some((k, n)) = kind {
            for _ in 0..n {
                advance(&mut i, &mut line, &mut col);
            }
            out.push(Token {
                kind: k,
                span: Span::new(start, i, sl, sc),
            });
            continue;
        }
        let k = match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '!' => TokenKind::Bang,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '=' => TokenKind::Eq,
            '&' => TokenKind::Amp,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            _ => {
                errors.push(Diagnostic {
                    code: None,
                    message: format!("unexpected character `{c}`"),
                    span: Span::new(start, start + 1, sl, sc),
                });
                advance(&mut i, &mut line, &mut col);
                continue;
            }
        };
        advance(&mut i, &mut line, &mut col);
        out.push(Token {
            kind: k,
            span: Span::new(start, i, sl, sc),
        });
    }
    out.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(i, i, line, col),
    });
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(StructDecl),
    Statement(Stmt),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Block,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    Expr(Expr),
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
    },
    While {
        condition: Expr,
        body: Block,
    },
    For {
        name: String,
        start: Expr,
        end: Expr,
        body: Block,
    },
    Break,
    Continue,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Borrow {
        mutable: bool,
        expr: Box<Expr>,
    },
    Int(String),
    Float(String),
    String(String),
    Bool(bool),
    Name(String),
    Array(Vec<Expr>),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Assign {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        base: Box<Expr>,
        name: String,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Name(String),
    Array(Box<Type>, usize),
    Reference { mutable: bool, inner: Box<Type> },
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<Diagnostic>,
}
impl Parser {
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn bump(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        t
    }
    fn check(&self, k: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(k)
    }
    fn expect(&mut self, k: TokenKind, msg: &str) -> bool {
        if self.check(&k) {
            self.bump();
            true
        } else {
            self.errors.push(Diagnostic {
                code: None,
                message: format!("expected {msg}, found {:?}", self.current().kind),
                span: self.current().span,
            });
            false
        }
    }
    fn ident(&mut self) -> Option<String> {
        match self.bump().kind {
            TokenKind::Ident(s) => Some(s),
            _ => {
                self.errors.push(Diagnostic {
                    code: None,
                    message: "expected identifier".into(),
                    span: self.current().span,
                });
                None
            }
        }
    }
    fn program(&mut self) -> Program {
        let mut items = Vec::new();
        while !self.check(&TokenKind::Eof) {
            if self.check(&TokenKind::Fn) {
                if let Some(x) = self.function() {
                    items.push(Item::Function(x));
                }
            } else if self.check(&TokenKind::Struct) {
                if let Some(x) = self.structure() {
                    items.push(Item::Struct(x));
                }
            } else if let Some(x) = self.statement() {
                items.push(Item::Statement(x));
            } else {
                self.bump();
            }
        }
        Program { items }
    }
    fn function(&mut self) -> Option<Function> {
        self.bump();
        let name = self.ident()?;
        self.expect(TokenKind::LParen, "`(`");
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
            let n = self.ident()?;
            self.expect(TokenKind::Colon, "`:`");
            let t = self.ty()?;
            params.push((n, t));
            if !self.check(&TokenKind::RParen) {
                self.expect(TokenKind::Comma, "`,`");
            }
        }
        self.expect(TokenKind::RParen, "`)`");
        let ret = if self.check(&TokenKind::Arrow) {
            self.bump();
            Some(self.ty()?)
        } else {
            None
        };
        Some(Function {
            name,
            params,
            return_type: ret,
            body: self.block()?,
        })
    }
    fn structure(&mut self) -> Option<StructDecl> {
        self.bump();
        let name = self.ident()?;
        self.expect(TokenKind::LBrace, "`{`");
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            let n = self.ident()?;
            self.expect(TokenKind::Colon, "`:`");
            let t = self.ty()?;
            fields.push((n, t));
            if !self.check(&TokenKind::RBrace) {
                self.expect(TokenKind::Comma, "`,`");
            }
        }
        self.expect(TokenKind::RBrace, "`}`");
        Some(StructDecl { name, fields })
    }
    fn ty(&mut self) -> Option<Type> {
        if self.check(&TokenKind::Amp) {
            self.bump();
            let mutable = self.check(&TokenKind::Mut);
            if mutable {
                self.bump();
            }
            return Some(Type::Reference {
                mutable,
                inner: Box::new(self.ty()?),
            });
        }
        if self.check(&TokenKind::LBracket) {
            self.bump();
            let t = self.ty()?;
            self.expect(TokenKind::Semicolon, "`;`");
            let n = match self.bump().kind {
                TokenKind::Int(s) => s.parse().unwrap_or(0),
                _ => {
                    self.errors.push(Diagnostic {
                        code: None,
                        message: "expected array length".into(),
                        span: self.current().span,
                    });
                    0
                }
            };
            self.expect(TokenKind::RBracket, "`]`");
            Some(Type::Array(Box::new(t), n))
        } else {
            let t = self.bump().kind;
            let s = match t {
                TokenKind::Ident(x) => x,
                TokenKind::Bool => "Bool".into(),
                TokenKind::I32 => "I32".into(),
                TokenKind::U32 => "U32".into(),
                TokenKind::U64 => "U64".into(),
                TokenKind::F64 => "F64".into(),
                TokenKind::StringType => "String".into(),
                _ => {
                    self.errors.push(Diagnostic {
                        code: None,
                        message: "expected type".into(),
                        span: self.current().span,
                    });
                    return None;
                }
            };
            Some(Type::Name(s))
        }
    }
    fn block(&mut self) -> Option<Block> {
        self.expect(TokenKind::LBrace, "`{`");
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            if let Some(s) = self.statement() {
                statements.push(s)
            } else {
                self.bump();
            }
        }
        self.expect(TokenKind::RBrace, "`}`");
        Some(Block { statements })
    }
    fn statement(&mut self) -> Option<Stmt> {
        if self.check(&TokenKind::Let) {
            self.bump();
            let mutable = self.check(&TokenKind::Mut);
            if mutable {
                self.bump();
            }
            let name = self.ident()?;
            let ty = if self.check(&TokenKind::Colon) {
                self.bump();
                Some(self.ty()?)
            } else {
                None
            };
            let value = if self.check(&TokenKind::Eq) {
                self.bump();
                Some(self.expr(0)?)
            } else {
                None
            };
            self.check(&TokenKind::Semicolon).then(|| self.bump());
            return Some(Stmt::Let {
                name,
                mutable,
                ty,
                value,
            });
        }
        if self.check(&TokenKind::Return) {
            self.bump();
            let e = if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::RBrace) {
                None
            } else {
                Some(self.expr(0)?)
            };
            self.check(&TokenKind::Semicolon).then(|| self.bump());
            return Some(Stmt::Return(e));
        }
        if self.check(&TokenKind::If) {
            self.bump();
            let c = self.expr(0)?;
            let t = self.block()?;
            let e = if self.check(&TokenKind::Else) {
                self.bump();
                Some(self.block()?)
            } else {
                None
            };
            return Some(Stmt::If {
                condition: c,
                then_block: t,
                else_block: e,
            });
        }
        if self.check(&TokenKind::While) {
            self.bump();
            let c = self.expr(0)?;
            return Some(Stmt::While {
                condition: c,
                body: self.block()?,
            });
        }
        if self.check(&TokenKind::For) {
            self.bump();
            let n = self.ident()?;
            self.expect(TokenKind::In, "`in`");
            let s = self.expr(0)?;
            self.expect(TokenKind::DotDot, "`..`");
            let e = self.expr(0)?;
            return Some(Stmt::For {
                name: n,
                start: s,
                end: e,
                body: self.block()?,
            });
        }
        if self.check(&TokenKind::Break) {
            self.bump();
            self.check(&TokenKind::Semicolon).then(|| self.bump());
            return Some(Stmt::Break);
        }
        if self.check(&TokenKind::Continue) {
            self.bump();
            self.check(&TokenKind::Semicolon).then(|| self.bump());
            return Some(Stmt::Continue);
        }
        let e = self.expr(0)?;
        self.check(&TokenKind::Semicolon).then(|| self.bump());
        Some(Stmt::Expr(e))
    }
    fn expr(&mut self, min: u8) -> Option<Expr> {
        let mut left = self.prefix()?;
        loop {
            let previous = left.clone();
            left = match self.postfix(left) {
                Some(x) => x,
                None => previous,
            };
            let (op, p) = match self.binop() {
                Some(x) => x,
                None => break,
            };
            if p < min {
                break;
            }
            self.bump();
            let right = self.expr(p + 1)?;
            left = if op == BinaryOp::Assign {
                Expr::Assign {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            } else {
                Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            };
        }
        Some(left)
    }
    fn prefix(&mut self) -> Option<Expr> {
        let t = self.bump();
        match t.kind {
            TokenKind::Int(x) => Some(Expr::Int(x)),
            TokenKind::Float(x) => Some(Expr::Float(x)),
            TokenKind::String(x) => Some(Expr::String(x)),
            TokenKind::True => Some(Expr::Bool(true)),
            TokenKind::False => Some(Expr::Bool(false)),
            TokenKind::Ident(x) => Some(Expr::Name(x)),
            TokenKind::Minus => Some(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(self.expr(10)?),
            }),
            TokenKind::Bang => Some(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(self.expr(10)?),
            }),
            TokenKind::Amp => {
                let mutable = self.check(&TokenKind::Mut);
                if mutable {
                    self.bump();
                }
                Some(Expr::Borrow {
                    mutable,
                    expr: Box::new(self.expr(10)?),
                })
            }
            TokenKind::Star => Some(Expr::Unary {
                op: UnaryOp::Deref,
                expr: Box::new(self.expr(10)?),
            }),
            TokenKind::LParen => {
                let x = self.expr(0);
                self.expect(TokenKind::RParen, "`)`");
                x
            }
            TokenKind::LBracket => {
                let mut a = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.check(&TokenKind::Eof) {
                    a.push(self.expr(0)?);
                    if !self.check(&TokenKind::RBracket) {
                        self.expect(TokenKind::Comma, "`,`");
                    }
                }
                self.expect(TokenKind::RBracket, "`]`");
                Some(Expr::Array(a))
            }
            _ => {
                self.errors.push(Diagnostic {
                    code: None,
                    message: "expected expression".into(),
                    span: t.span,
                });
                None
            }
        }
    }
    fn postfix(&mut self, mut x: Expr) -> Option<Expr> {
        loop {
            if self.check(&TokenKind::LParen) {
                self.bump();
                let mut a = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    a.push(self.expr(0)?);
                    if !self.check(&TokenKind::RParen) {
                        self.expect(TokenKind::Comma, "`,`");
                    }
                }
                self.expect(TokenKind::RParen, "`)`");
                x = Expr::Call {
                    callee: Box::new(x),
                    args: a,
                };
            } else if self.check(&TokenKind::LBracket) {
                self.bump();
                let i = self.expr(0)?;
                self.expect(TokenKind::RBracket, "`]`");
                x = Expr::Index {
                    base: Box::new(x),
                    index: Box::new(i),
                };
            } else if self.check(&TokenKind::Dot) {
                self.bump();
                x = Expr::Field {
                    base: Box::new(x),
                    name: self.ident()?,
                };
            } else {
                break;
            }
        }
        Some(x)
    }
    fn binop(&self) -> Option<(BinaryOp, u8)> {
        Some(match self.current().kind {
            TokenKind::Eq => (BinaryOp::Assign, 1),
            TokenKind::OrOr => (BinaryOp::Or, 2),
            TokenKind::AndAnd => (BinaryOp::And, 3),
            TokenKind::EqEq => (BinaryOp::Eq, 4),
            TokenKind::NotEq => (BinaryOp::NotEq, 4),
            TokenKind::Lt => (BinaryOp::Lt, 5),
            TokenKind::LtEq => (BinaryOp::LtEq, 5),
            TokenKind::Gt => (BinaryOp::Gt, 5),
            TokenKind::GtEq => (BinaryOp::GtEq, 5),
            TokenKind::Plus => (BinaryOp::Add, 6),
            TokenKind::Minus => (BinaryOp::Sub, 6),
            TokenKind::Star => (BinaryOp::Mul, 7),
            TokenKind::Slash => (BinaryOp::Div, 7),
            TokenKind::Percent => (BinaryOp::Rem, 7),
            _ => return None,
        })
    }
}
pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<Diagnostic>> {
    let mut p = Parser {
        tokens,
        pos: 0,
        errors: Vec::new(),
    };
    let ast = p.program();
    if p.errors.is_empty() {
        Ok(ast)
    } else {
        Err(p.errors)
    }
}
pub fn parse_source(source: &str) -> Result<Program, Vec<Diagnostic>> {
    parse(lex(source)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexes_locations() {
        let t = lex("let x = 42;").unwrap();
        assert_eq!(t[0].kind, TokenKind::Let);
        assert_eq!(t[1].span.column, 5);
    }
    #[test]
    fn parses_frontend_surface() {
        let p=parse_source("struct Point { x: I32, y: I32 } fn main() { let mut p = [1, 2]; if p[0] == 1 { p[0] = 3; } }").unwrap();
        assert_eq!(p.items.len(), 2);
    }
    #[test]
    fn malformed_is_error() {
        assert!(parse_source("fn main( {").is_err());
        assert!(lex("\"").is_err());
    }
}

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticType {
    Bool,
    I32,
    U32,
    U64,
    F64,
    String,
    Reference {
        mutable: bool,
        inner: Box<SemanticType>,
    },
    Array(Box<SemanticType>, usize),
    Struct(String),
    Unit,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum OwnershipState {
    Uninitialized,
    Owned,
    Moved,
}
#[derive(Clone, Debug)]
struct Binding {
    ty: SemanticType,
    mutable: bool,
    initialized: bool,
    ownership: OwnershipState,
}
#[derive(Clone, Debug)]
struct BorrowState {
    mutable: bool,
    scope_depth: usize,
    persistent: bool,
}
#[derive(Clone, Debug)]
struct FunctionSignature {
    params: Vec<SemanticType>,
    return_type: SemanticType,
}
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, Binding>>,
    functions: HashMap<String, FunctionSignature>,
    structs: HashMap<String, HashMap<String, SemanticType>>,
    errors: Vec<Diagnostic>,
    current_return: SemanticType,
    loop_depth: usize,
    borrow_states: HashMap<String, BorrowState>,
    scope_depth: usize,
    source: Option<String>,
}
impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "print".into(),
            FunctionSignature {
                params: vec![SemanticType::String],
                return_type: SemanticType::Unit,
            },
        );
        Self {
            scopes: vec![HashMap::new()],
            functions,
            structs: HashMap::new(),
            errors: Vec::new(),
            current_return: SemanticType::Unit,
            loop_depth: 0,
            borrow_states: HashMap::new(),
            scope_depth: 0,
            source: None,
        }
    }
    fn error(&mut self, message: impl Into<String>) {
        let message = message.into();
        let code = if message.contains("move") {
            "E0401"
        } else if message.contains("borrow") || message.contains("borrowed") {
            "E0402"
        } else if message.contains("uninitialized") {
            "E0403"
        } else if message.contains("immutable") || message.contains("assign") {
            "E0404"
        } else if message.contains("type") {
            "E0301"
        } else {
            "E0001"
        };
        let span = self
            .source
            .as_deref()
            .and_then(|source| {
                let needle = message.split('`').nth(1)?;
                let start = source.rfind(needle)?;
                let before = &source[..start];
                Some(Span::new(
                    start,
                    start + needle.len(),
                    before.lines().count().max(1),
                    start - before.rfind('\n').map(|i| i + 1).unwrap_or(0) + 1,
                ))
            })
            .unwrap_or_else(|| Span::new(0, 0, 1, 1));
        self.errors.push(Diagnostic {
            code: Some(code),
            message,
            span,
        });
    }
    fn enter(&mut self) {
        self.scope_depth += 1;
        self.scopes.push(HashMap::new());
    }
    fn leave(&mut self) {
        self.scopes.pop();
        self.borrow_states
            .retain(|_, borrow| borrow.scope_depth < self.scope_depth);
        self.scope_depth = self.scope_depth.saturating_sub(1);
    }
    fn declare(&mut self, name: &str, binding: Binding) {
        let scope = self
            .scopes
            .last_mut()
            .expect("semantic analyzer always has a scope");
        if scope.contains_key(name) {
            self.error(format!("duplicate definition of `{name}`"));
        } else {
            scope.insert(name.to_string(), binding);
        }
    }
    fn lookup(&self, name: &str) -> Option<Binding> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }
    fn lookup_mut(&mut self, name: &str) -> Option<&mut Binding> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }
    fn type_from_ast(&mut self, ty: &Type) -> SemanticType {
        match ty {
            Type::Name(name) => match name.as_str() {
                "Bool" => SemanticType::Bool,
                "I32" => SemanticType::I32,
                "U32" => SemanticType::U32,
                "U64" => SemanticType::U64,
                "F64" => SemanticType::F64,
                "String" => SemanticType::String,
                other => SemanticType::Struct(other.into()),
            },
            Type::Array(element, length) => {
                SemanticType::Array(Box::new(self.type_from_ast(element)), *length)
            }
            Type::Reference { mutable, inner } => SemanticType::Reference {
                mutable: *mutable,
                inner: Box::new(self.type_from_ast(inner)),
            },
        }
    }
    fn compatible(expected: &SemanticType, found: &SemanticType) -> bool {
        expected == found || *expected == SemanticType::Unknown || *found == SemanticType::Unknown
    }
    fn is_copy(&self, ty: &SemanticType) -> bool {
        match ty {
            SemanticType::Bool
            | SemanticType::I32
            | SemanticType::U32
            | SemanticType::U64
            | SemanticType::F64
            | SemanticType::Reference { .. } => true,
            SemanticType::Array(element, _) => self.is_copy(element),
            SemanticType::Struct(name) => self
                .structs
                .get(name)
                .map(|fields| fields.values().all(|field| self.is_copy(field)))
                .unwrap_or(false),
            SemanticType::String | SemanticType::Unit | SemanticType::Unknown => false,
        }
    }
    fn ensure_available(&mut self, name: &str, action: &str) -> Option<Binding> {
        match self.lookup(name) {
            Some(binding) if binding.ownership == OwnershipState::Moved => {
                self.error(format!(
                    "use after move of `{name}` while attempting to {action}"
                ));
                None
            }
            Some(binding)
                if !binding.initialized || binding.ownership == OwnershipState::Uninitialized =>
            {
                self.error(format!("use of uninitialized binding `{name}`"));
                None
            }
            Some(binding) => Some(binding),
            None => {
                self.error(format!("undefined name `{name}`"));
                None
            }
        }
    }
    fn ensure_not_borrowed(&mut self, name: &str, action: &str) {
        if let Some(borrow) = self.borrow_states.get(name) {
            self.error(if borrow.mutable {
                format!("cannot {action} `{name}`: an exclusive mutable borrow is active")
            } else {
                format!("cannot {action} `{name}`: a shared borrow is active")
            });
        }
    }
    fn move_expr(&mut self, expr: &Expr) {
        let Expr::Name(name) = expr else {
            return;
        };
        let Some(current) = self.lookup(name) else {
            return;
        };
        if current.ownership == OwnershipState::Moved || !current.initialized {
            return;
        }
        let Some(binding) = self.ensure_available(name, "move") else {
            return;
        };
        if self.is_copy(&binding.ty) {
            return;
        }
        self.ensure_not_borrowed(name, "move");
        if let Some(binding) = self.lookup_mut(name) {
            binding.ownership = OwnershipState::Moved;
        }
    }
    fn release_temporaries(&mut self) {
        self.borrow_states.retain(|_, borrow| borrow.persistent);
    }
    fn mark_borrow(&mut self, name: &str, mutable: bool) {
        if let Some(existing) = self.borrow_states.get(name) {
            if mutable || existing.mutable {
                self.error(if existing.mutable {
                    format!("cannot create a second mutable borrow of `{name}`")
                } else {
                    format!("cannot mutably borrow `{name}` while a shared borrow is active")
                });
            }
            return;
        }
        self.borrow_states.insert(
            name.to_string(),
            BorrowState {
                mutable,
                scope_depth: self.scope_depth,
                persistent: false,
            },
        );
    }
    fn persist_borrow(&mut self, expr: &Expr) {
        if let Expr::Borrow { expr, .. } = expr {
            if let Expr::Name(name) = expr.as_ref() {
                if let Some(borrow) = self.borrow_states.get_mut(name) {
                    borrow.persistent = true;
                }
            }
        }
    }
    pub fn analyze(mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        self.collect_structs(program);
        self.collect_functions(program);
        for item in &program.items {
            if let Item::Struct(decl) = item {
                self.check_struct(decl);
            }
        }
        for item in &program.items {
            match item {
                Item::Function(function) => self.check_function(function),
                Item::Statement(stmt) => self.check_stmt(stmt),
                Item::Struct(_) => {}
            }
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
    fn collect_structs(&mut self, program: &Program) {
        for item in &program.items {
            if let Item::Struct(decl) = item {
                if self.structs.contains_key(&decl.name) || self.functions.contains_key(&decl.name)
                {
                    self.error(format!("duplicate definition of `{}`", decl.name));
                } else {
                    self.structs.insert(decl.name.clone(), HashMap::new());
                }
            }
        }
    }
    fn collect_functions(&mut self, program: &Program) {
        for item in &program.items {
            if let Item::Function(function) = item {
                if self.functions.contains_key(&function.name)
                    || self.structs.contains_key(&function.name)
                {
                    self.error(format!("duplicate definition of `{}`", function.name));
                    continue;
                }
                let params = function
                    .params
                    .iter()
                    .map(|(_, ty)| self.type_from_ast(ty))
                    .collect();
                let return_type = function
                    .return_type
                    .as_ref()
                    .map(|ty| self.type_from_ast(ty))
                    .unwrap_or(SemanticType::Unit);
                self.functions.insert(
                    function.name.clone(),
                    FunctionSignature {
                        params,
                        return_type,
                    },
                );
            }
        }
    }
    fn check_struct(&mut self, decl: &StructDecl) {
        let mut fields = HashMap::new();
        for (name, ty) in &decl.fields {
            if fields.contains_key(name) {
                self.error(format!(
                    "duplicate field `{name}` in struct `{}`",
                    decl.name
                ));
            }
            fields.insert(name.clone(), self.type_from_ast(ty));
        }
        if let Some(existing) = self.structs.get_mut(&decl.name) {
            *existing = fields;
        }
    }
    fn check_function(&mut self, function: &Function) {
        self.enter();
        let function_return = function
            .return_type
            .as_ref()
            .map(|ty| self.type_from_ast(ty))
            .unwrap_or(SemanticType::Unit);
        if matches!(function_return, SemanticType::Reference { .. }) {
            self.error(format!(
                "function `{}` cannot return a reference in the current ownership milestone",
                function.name
            ));
        }
        let previous_return = std::mem::replace(&mut self.current_return, function_return);
        for (name, ty) in &function.params {
            let parameter_type = self.type_from_ast(ty);
            self.declare(
                name,
                Binding {
                    ty: parameter_type,
                    mutable: false,
                    initialized: true,
                    ownership: OwnershipState::Owned,
                },
            );
        }
        self.check_block(&function.body);
        self.current_return = previous_return;
        self.leave();
    }
    fn check_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.check_stmt(stmt);
        }
    }
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                ty,
                value,
            } => {
                let declared = ty.as_ref().map(|t| self.type_from_ast(t));
                let inferred = value
                    .as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(SemanticType::Unknown);
                let final_type = declared.clone().unwrap_or_else(|| inferred.clone());
                if value.is_none() && declared.is_none() {
                    self.error(format!("binding `{name}` needs a type or initializer"));
                }
                if let Some(expected) = declared {
                    if !Self::compatible(&expected, &inferred) {
                        self.error(format!(
                            "type mismatch in `{name}`: expected {:?}, found {:?}",
                            expected, inferred
                        ));
                    }
                }
                if let Some(value) = value {
                    self.move_expr(value);
                    self.persist_borrow(value);
                }
                self.declare(
                    name,
                    Binding {
                        ty: final_type,
                        mutable: *mutable,
                        initialized: value.is_some(),
                        ownership: if value.is_some() {
                            OwnershipState::Owned
                        } else {
                            OwnershipState::Uninitialized
                        },
                    },
                );
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr);
            }
            Stmt::Return(expr) => {
                let found = expr
                    .as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(SemanticType::Unit);
                if !Self::compatible(&self.current_return, &found) {
                    self.error(format!(
                        "return type mismatch: expected {:?}, found {:?}",
                        self.current_return, found
                    ));
                }
                if let Some(e) = expr {
                    self.move_expr(e);
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let condition_type = self.check_expr(condition);
                self.require_type(condition_type, SemanticType::Bool, "if condition");
                let before = self.scopes.clone();
                self.enter();
                self.check_block(then_block);
                self.leave();
                let then_state = self.scopes.clone();
                if let Some(block) = else_block {
                    self.scopes = before.clone();
                    self.enter();
                    self.check_block(block);
                    self.leave();
                    let else_state = self.scopes.clone();
                    self.merge_initialized(&before, &then_state, &else_state);
                } else {
                    self.scopes = before;
                }
            }
            Stmt::While { condition, body } => {
                let condition_type = self.check_expr(condition);
                self.require_type(condition_type, SemanticType::Bool, "while condition");
                let before = self.scopes.clone();
                self.loop_depth += 1;
                self.enter();
                self.check_block(body);
                self.leave();
                self.loop_depth -= 1;
                self.scopes = before;
            }
            Stmt::For {
                name,
                start,
                end,
                body,
            } => {
                let start_type = self.check_expr(start);
                let end_type = self.check_expr(end);
                self.require_type(start_type, SemanticType::I32, "for range start");
                self.require_type(end_type, SemanticType::I32, "for range end");
                let before = self.scopes.clone();
                self.loop_depth += 1;
                self.enter();
                self.declare(
                    name,
                    Binding {
                        ty: SemanticType::I32,
                        mutable: false,
                        initialized: true,
                        ownership: OwnershipState::Owned,
                    },
                );
                self.check_block(body);
                self.leave();
                self.loop_depth -= 1;
                self.scopes = before;
            }
            Stmt::Break | Stmt::Continue => {
                if self.loop_depth == 0 {
                    self.error("`break` and `continue` are only valid inside loops");
                }
            }
        }
        self.release_temporaries();
    }
    fn merge_initialized(
        &mut self,
        base: &[HashMap<String, Binding>],
        left: &[HashMap<String, Binding>],
        right: &[HashMap<String, Binding>],
    ) {
        for (index, scope) in self.scopes.iter_mut().enumerate() {
            for (name, binding) in scope.iter_mut() {
                if let (Some(l), Some(r)) = (
                    left.get(index).and_then(|s| s.get(name)),
                    right.get(index).and_then(|s| s.get(name)),
                ) {
                    binding.initialized = l.initialized && r.initialized;
                    if l.ownership == OwnershipState::Moved || r.ownership == OwnershipState::Moved
                    {
                        binding.ownership = OwnershipState::Moved;
                    }
                } else if let Some(original) = base.get(index).and_then(|s| s.get(name)) {
                    binding.initialized = original.initialized;
                    binding.ownership = original.ownership.clone();
                }
            }
        }
    }
    fn require_type(&mut self, found: SemanticType, expected: SemanticType, context: &str) {
        if !Self::compatible(&expected, &found) {
            self.error(format!(
                "{context} must be {:?}, found {:?}",
                expected, found
            ));
        }
    }
    fn check_expr(&mut self, expr: &Expr) -> SemanticType {
        match expr {
            Expr::Borrow { mutable, expr } => {
                let inner = self.check_expr(expr);
                if let Expr::Name(name) = expr.as_ref() {
                    if let Some(binding) = self.ensure_available(name, "borrow") {
                        if *mutable && !binding.mutable {
                            self.error(format!("cannot mutably borrow immutable binding `{name}`"));
                        }
                        self.ensure_not_borrowed(name, "borrow");
                        self.mark_borrow(name, *mutable);
                    }
                }
                SemanticType::Reference {
                    mutable: *mutable,
                    inner: Box::new(inner),
                }
            }
            Expr::Int(value) => {
                match value.parse::<i64>() {
                    Ok(number) if (0..=i32::MAX as i64).contains(&number) => {}
                    _ => self.error(format!(
                        "integer literal `{value}` is outside the supported I32 range"
                    )),
                }
                SemanticType::I32
            }
            Expr::Float(_) => SemanticType::F64,
            Expr::String(_) => SemanticType::String,
            Expr::Bool(_) => SemanticType::Bool,
            Expr::Name(name) => self
                .ensure_available(name, "read")
                .map(|b| b.ty)
                .unwrap_or(SemanticType::Unknown),
            Expr::Array(items) => {
                let mut element = SemanticType::Unknown;
                for item in items {
                    let ty = self.check_expr(item);
                    if element == SemanticType::Unknown {
                        element = ty;
                    } else if !Self::compatible(&element, &ty) {
                        self.error("array elements must have the same type");
                    }
                }
                SemanticType::Array(Box::new(element), items.len())
            }
            Expr::Unary { op, expr } => {
                let ty = self.check_expr(expr);
                match op {
                    UnaryOp::Neg => {
                        if !matches!(
                            ty,
                            SemanticType::I32
                                | SemanticType::U32
                                | SemanticType::U64
                                | SemanticType::F64
                                | SemanticType::Unknown
                        ) {
                            self.error("negation requires a numeric operand");
                        }
                        ty
                    }
                    UnaryOp::Not => {
                        self.require_type(ty, SemanticType::Bool, "logical negation");
                        SemanticType::Bool
                    }
                    UnaryOp::Deref => match ty {
                        SemanticType::Reference { inner, .. } => *inner,
                        SemanticType::Unknown => SemanticType::Unknown,
                        _ => {
                            self.error("dereference requires a reference");
                            SemanticType::Unknown
                        }
                    },
                }
            }
            Expr::Binary { left, op, right } => {
                let l = self.check_expr(left);
                let r = self.check_expr(right);
                match op {
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Rem => {
                        if !matches!(
                            l,
                            SemanticType::I32
                                | SemanticType::U32
                                | SemanticType::U64
                                | SemanticType::F64
                                | SemanticType::Unknown
                        ) || !Self::compatible(&l, &r)
                        {
                            self.error("arithmetic operands must have the same numeric type");
                        }
                        l
                    }
                    BinaryOp::Eq
                    | BinaryOp::NotEq
                    | BinaryOp::Lt
                    | BinaryOp::LtEq
                    | BinaryOp::Gt
                    | BinaryOp::GtEq => {
                        if !Self::compatible(&l, &r) {
                            self.error("comparison operands must have compatible types");
                        }
                        SemanticType::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        self.require_type(l, SemanticType::Bool, "logical operand");
                        self.require_type(r, SemanticType::Bool, "logical operand");
                        SemanticType::Bool
                    }
                    BinaryOp::Assign => SemanticType::Unknown,
                }
            }
            Expr::Assign { left, right } => {
                let right_ty = self.check_expr(right);
                match left.as_ref() {
                    Expr::Name(name) => {
                        self.ensure_not_borrowed(name, "assign to");
                        if let Some(binding) = self.lookup(name) {
                            if !binding.mutable {
                                self.error(format!("cannot assign to immutable binding `{name}"));
                            }
                            if !Self::compatible(&binding.ty, &right_ty) {
                                self.error(format!(
                                    "assignment type mismatch: expected {:?}, found {:?}",
                                    binding.ty, right_ty
                                ));
                            }
                            let result_ty = binding.ty.clone();
                            self.move_expr(right);
                            if let Some(binding) = self.lookup_mut(name) {
                                binding.initialized = true;
                                binding.ownership = OwnershipState::Owned;
                            }
                            result_ty
                        } else {
                            self.error(format!("undefined name `{name}"));
                            SemanticType::Unknown
                        }
                    }
                    Expr::Unary {
                        op: UnaryOp::Deref,
                        expr,
                    } => {
                        let ref_ty = self.check_expr(expr);
                        match ref_ty {
                            SemanticType::Reference { mutable, inner } => {
                                if !mutable {
                                    self.error("cannot assign through a shared reference");
                                }
                                if !Self::compatible(&inner, &right_ty) {
                                    self.error(format!(
                                        "assignment type mismatch: expected {:?}, found {:?}",
                                        inner, right_ty
                                    ));
                                }
                                *inner
                            }
                            _ => {
                                self.error("dereference assignment requires a reference");
                                SemanticType::Unknown
                            }
                        }
                    }
                    Expr::Index { base, .. } | Expr::Field { base, .. } => {
                        if let Expr::Name(name) = base.as_ref() {
                            self.ensure_not_borrowed(name, "assign to");
                            if let Some(binding) = self.lookup(name) {
                                if !binding.mutable {
                                    self.error(format!(
                                        "cannot assign through immutable binding `{name}"
                                    ));
                                }
                            } else {
                                self.error(format!("undefined name `{name}"));
                            }
                            if let Some(binding) = self.lookup_mut(name) {
                                binding.initialized = true;
                                binding.ownership = OwnershipState::Owned;
                            }
                        } else {
                            self.error("aggregate assignment requires a named local");
                        }
                        right_ty
                    }
                    _ => {
                        self.error("left side of assignment is not assignable");
                        right_ty
                    }
                }
            }
            Expr::Call { callee, args } => {
                let signature = match callee.as_ref() {
                    Expr::Name(name) => self.functions.get(name).cloned().or_else(|| {
                        self.error(format!("undefined function `{name}`"));
                        None
                    }),
                    _ => {
                        self.error("only named functions are callable in v0.1");
                        None
                    }
                };
                let arg_types: Vec<SemanticType> =
                    args.iter().map(|arg| self.check_expr(arg)).collect();
                if let Some(sig) = signature {
                    if args.len() != sig.params.len() {
                        self.error(format!(
                            "function expects {} argument(s), found {}",
                            sig.params.len(),
                            args.len()
                        ));
                    }
                    for ((arg, found), expected) in
                        args.iter().zip(arg_types.iter()).zip(sig.params.iter())
                    {
                        if !Self::compatible(expected, found) {
                            self.error(format!(
                                "argument type mismatch: expected {:?}, found {:?}",
                                expected, found
                            ));
                        }
                        if !matches!(expected, SemanticType::Reference { .. }) {
                            self.move_expr(arg);
                        }
                    }
                    sig.return_type
                } else {
                    SemanticType::Unknown
                }
            }
            Expr::Index { base, index } => {
                let base_ty = self.check_expr(base);
                let index_type = self.check_expr(index);
                self.require_type(index_type, SemanticType::I32, "array index");
                match base_ty {
                    SemanticType::Array(element, _) => *element,
                    SemanticType::Unknown => SemanticType::Unknown,
                    _ => {
                        self.error("indexing requires an array");
                        SemanticType::Unknown
                    }
                }
            }
            Expr::Field { base, name } => {
                let base_ty = self.check_expr(base);
                match base_ty {
                    SemanticType::Struct(struct_name) => self
                        .structs
                        .get(&struct_name)
                        .and_then(|fields| fields.get(name))
                        .cloned()
                        .unwrap_or_else(|| {
                            self.error(format!("struct `{struct_name}` has no field `{name}`"));
                            SemanticType::Unknown
                        }),
                    SemanticType::Unknown => SemanticType::Unknown,
                    _ => {
                        self.error("field access requires a struct");
                        SemanticType::Unknown
                    }
                }
            }
        }
    }
}
pub fn analyze(program: &Program) -> Result<(), Vec<Diagnostic>> {
    SemanticAnalyzer::new().analyze(program)
}
pub fn analyze_with_source(program: &Program, source: &str) -> Result<(), Vec<Diagnostic>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.source = Some(source.to_string());
    analyzer.analyze(program)
}
#[cfg(test)]
mod semantic_tests {
    use super::*;
    fn check(source: &str) -> Result<(), Vec<Diagnostic>> {
        analyze(&parse_source(source).expect("test source must parse"))
    }
    #[test]
    fn resolves_nested_scopes() {
        assert!(check("fn main() { let x: I32 = 1; if true { let y: I32 = x; } }").is_ok());
    }
    #[test]
    fn rejects_undefined_names() {
        let e = check("fn main() { let x = missing; }").unwrap_err();
        assert!(e
            .iter()
            .any(|d| d.message.contains("undefined name `missing`")));
    }
    #[test]
    fn rejects_uninitialized_reads() {
        let e = check("fn main() -> I32 { let x: I32; return x; }").unwrap_err();
        assert!(e.iter().any(|d| d.message.contains("uninitialized")));
    }
    #[test]
    fn joins_initialization_across_if_branches() {
        assert!(check(
            "fn main() -> I32 { let mut x: I32; if true { x = 1; } else { x = 2; } return x; }"
        )
        .is_ok());
        let e =
            check("fn main() -> I32 { let mut x: I32; if true { x = 1; } return x; }").unwrap_err();
        assert!(e.iter().any(|d| d.message.contains("uninitialized")));
    }
    #[test]
    fn rejects_integer_literal_overflow() {
        let e = check("fn main() { let x: I32 = 2147483648; }").unwrap_err();
        assert!(e
            .iter()
            .any(|d| d.message.contains("outside the supported I32 range")));
    }
    #[test]
    fn rejects_duplicates_and_immutable_assignment() {
        let e = check("fn main() { let x = 1; let x = 2; x = 3; }").unwrap_err();
        assert!(e.iter().any(|d| d.message.contains("duplicate definition")));
        assert!(e.iter().any(|d| d.message.contains("immutable")));
    }
    #[test]
    fn checks_functions_and_structs() {
        let source = "struct Point { x: I32, y: I32 } fn add(a: I32, b: I32) -> I32 { return a + b; } fn main() { let p: Point; let x: I32 = add(1, 2); }";
        assert!(check(source).is_ok());
    }
    #[test]
    fn rejects_bad_operator_and_call() {
        let e = check("fn add(a: I32) -> I32 { return a; } fn main() { let x = add(true); }")
            .unwrap_err();
        assert!(e
            .iter()
            .any(|d| d.message.contains("argument type mismatch")));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenError {
    pub message: String,
}

#[derive(Clone, Debug)]
struct Local {
    ptr: String,
    ty: SemanticType,
}

struct LlvmCodegen {
    output: String,
    globals: String,
    temp: usize,
    locals: HashMap<String, Local>,
    struct_fields: HashMap<String, Vec<(String, SemanticType)>>,
    break_labels: Vec<String>,
    continue_labels: Vec<String>,
    string_id: usize,
    current_return: SemanticType,
    function_returns: HashMap<String, SemanticType>,
}

impl LlvmCodegen {
    fn new() -> Self {
        Self {
            output: String::new(),
            globals: String::new(),
            temp: 0,
            locals: HashMap::new(),
            struct_fields: HashMap::new(),
            break_labels: Vec::new(),
            continue_labels: Vec::new(),
            string_id: 0,
            current_return: SemanticType::Unit,
            function_returns: HashMap::new(),
        }
    }
    fn next_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp);
        self.temp += 1;
        name
    }
    fn next_label(&mut self, prefix: &str) -> String {
        let label = format!("{}.{}", prefix, self.temp);
        self.temp += 1;
        label
    }
    fn emit_bounds_check(&mut self, index: &str, length: usize) {
        let ok_label = self.next_label("bounds.ok");
        let fail_label = self.next_label("bounds.fail");
        let condition = self.next_temp();
        self.output.push_str(&format!("  {condition} = icmp ult i32 {index}, {length}\n  br i1 {condition}, label %{ok_label}, label %{fail_label}\n\n{fail_label}:\n  call void @llvm.trap()\n  unreachable\n\n{ok_label}:\n"));
    }
    fn llvm_type(ty: &SemanticType) -> Result<String, CodegenError> {
        match ty {
            SemanticType::Bool => Ok("i1".into()),
            SemanticType::I32 | SemanticType::U32 => Ok("i32".into()),
            SemanticType::U64 => Ok("i64".into()),
            SemanticType::F64 => Ok("double".into()),
            SemanticType::String => Ok("ptr".into()),
            SemanticType::Reference { .. } => Ok("ptr".into()),
            SemanticType::Unit => Ok("void".into()),
            SemanticType::Array(element, length) => {
                Ok(format!("[{} x {}]", length, Self::llvm_type(element)?))
            }
            SemanticType::Struct(name) => Ok(format!("%struct.{name}")),
            _ => Err(CodegenError {
                message: format!("LLVM lowering for {:?} is not implemented", ty),
            }),
        }
    }
    fn ast_type(ty: &Type) -> SemanticType {
        match ty {
            Type::Name(name) => match name.as_str() {
                "Bool" => SemanticType::Bool,
                "I32" => SemanticType::I32,
                "U32" => SemanticType::U32,
                "U64" => SemanticType::U64,
                "F64" => SemanticType::F64,
                "String" => SemanticType::String,
                other => SemanticType::Struct(other.into()),
            },
            Type::Array(element, length) => {
                SemanticType::Array(Box::new(Self::ast_type(element)), *length)
            }
            Type::Reference { mutable, inner } => SemanticType::Reference {
                mutable: *mutable,
                inner: Box::new(Self::ast_type(inner)),
            },
        }
    }
    fn emit_lvalue(&mut self, expr: &Expr) -> Result<(String, SemanticType), CodegenError> {
        match expr {
            Expr::Name(name) => self
                .locals
                .get(name)
                .map(|l| (l.ptr.clone(), l.ty.clone()))
                .ok_or_else(|| CodegenError {
                    message: format!("no local storage for `{name}`"),
                }),
            Expr::Unary {
                op: UnaryOp::Deref,
                expr,
            } => {
                let (pointer, ty) = self.emit_expr(expr)?;
                match ty {
                    SemanticType::Reference { inner, .. } => Ok((pointer, *inner)),
                    _ => Err(CodegenError {
                        message: "dereference requires a reference".into(),
                    }),
                }
            }
            _ => Err(CodegenError {
                message: "reference target must be a local or dereference".into(),
            }),
        }
    }
    fn emit_expr(&mut self, expr: &Expr) -> Result<(String, SemanticType), CodegenError> {
        match expr {
            Expr::Borrow { mutable, expr } => {
                let (address, inner) = self.emit_lvalue(expr)?;
                Ok((
                    address,
                    SemanticType::Reference {
                        mutable: *mutable,
                        inner: Box::new(inner),
                    },
                ))
            }
            Expr::Int(_) => Ok((
                match expr {
                    Expr::Int(value) => value.clone(),
                    _ => unreachable!(),
                },
                SemanticType::I32,
            )),
            Expr::Float(value) => Ok((
                format!(
                    "0x{:016X}",
                    value
                        .parse::<f64>()
                        .map_err(|_| CodegenError {
                            message: format!("invalid floating literal `{value}`")
                        })?
                        .to_bits()
                ),
                SemanticType::F64,
            )),
            Expr::Bool(value) => Ok((
                if *value { "1".into() } else { "0".into() },
                SemanticType::Bool,
            )),
            Expr::String(value) => {
                let id = self.string_id;
                self.string_id += 1;
                let bytes = value.as_bytes();
                let mut escaped = String::new();
                for byte in bytes {
                    if byte.is_ascii_graphic() && *byte != b'"' && *byte != b'\\' {
                        escaped.push(*byte as char);
                    } else {
                        escaped.push_str(&format!("\\{:02X}", byte));
                    }
                }
                escaped.push_str("\\00");
                let length = bytes.len() + 1;
                self.globals.push_str(&format!(
                    "@.str.{id} = private unnamed_addr constant [{length} x i8] c\"{escaped}\"\n"
                ));
                let value_ptr = self.next_temp();
                self.output.push_str(&format!("  {value_ptr} = getelementptr inbounds [{length} x i8], ptr @.str.{id}, i64 0, i64 0\n"));
                Ok((value_ptr, SemanticType::String))
            }
            Expr::Name(name) => {
                let local = self.locals.get(name).cloned().ok_or_else(|| CodegenError {
                    message: format!("no local storage for `{name}`"),
                })?;
                if matches!(local.ty, SemanticType::Reference { .. }) {
                    return Ok((local.ptr, local.ty));
                }
                let value = self.next_temp();
                self.output.push_str(&format!(
                    "  {value} = load {}, ptr {}\n",
                    Self::llvm_type(&local.ty)?,
                    local.ptr
                ));
                Ok((value, local.ty))
            }
            Expr::Unary { op, expr } => {
                if matches!(op, UnaryOp::Deref) {
                    let (pointer, SemanticType::Reference { inner, .. }) = self.emit_expr(expr)?
                    else {
                        return Err(CodegenError {
                            message: "dereference requires a reference".into(),
                        });
                    };
                    let result = self.next_temp();
                    let llvm_ty = Self::llvm_type(&inner)?;
                    self.output
                        .push_str(&format!("  {result} = load {llvm_ty}, ptr {pointer}\n"));
                    return Ok((result, *inner));
                }
                let (value, ty) = self.emit_expr(expr)?;
                let result = self.next_temp();
                match op {
                    UnaryOp::Neg => {
                        let inst = if ty == SemanticType::F64 {
                            "fneg"
                        } else {
                            "sub"
                        };
                        if ty == SemanticType::F64 {
                            self.output
                                .push_str(&format!("  {result} = fneg double {value}\n"));
                        } else {
                            self.output.push_str(&format!(
                                "  {result} = sub {} 0, {value}\n",
                                Self::llvm_type(&ty)?
                            ));
                        }
                        let _ = inst;
                    }
                    UnaryOp::Not => self
                        .output
                        .push_str(&format!("  {result} = xor i1 {value}, true\n")),
                    UnaryOp::Deref => unreachable!(),
                }
                Ok((result, ty))
            }
            Expr::Binary { left, op, right } => {
                let (lv, lt) = self.emit_expr(left)?;
                let (rv, rt) = self.emit_expr(right)?;
                if lt != rt {
                    return Err(CodegenError {
                        message: "binary operands have incompatible LLVM types".into(),
                    });
                }
                let result = self.next_temp();
                let llvm_ty = Self::llvm_type(&lt)?;
                let instruction = match op {
                    BinaryOp::Add => {
                        if lt == SemanticType::F64 {
                            "fadd"
                        } else {
                            "add"
                        }
                    }
                    BinaryOp::Sub => {
                        if lt == SemanticType::F64 {
                            "fsub"
                        } else {
                            "sub"
                        }
                    }
                    BinaryOp::Mul => {
                        if lt == SemanticType::F64 {
                            "fmul"
                        } else {
                            "mul"
                        }
                    }
                    BinaryOp::Div => {
                        if lt == SemanticType::F64 {
                            "fdiv"
                        } else {
                            "sdiv"
                        }
                    }
                    BinaryOp::Rem => {
                        if lt == SemanticType::F64 {
                            " frem"
                        } else {
                            "srem"
                        }
                    }
                    BinaryOp::Eq => {
                        if lt == SemanticType::F64 {
                            "fcmp oeq"
                        } else {
                            "icmp eq"
                        }
                    }
                    BinaryOp::NotEq => {
                        if lt == SemanticType::F64 {
                            "fcmp one"
                        } else {
                            "icmp ne"
                        }
                    }
                    BinaryOp::Lt => {
                        if lt == SemanticType::F64 {
                            "fcmp olt"
                        } else {
                            "icmp slt"
                        }
                    }
                    BinaryOp::LtEq => {
                        if lt == SemanticType::F64 {
                            "fcmp ole"
                        } else {
                            "icmp sle"
                        }
                    }
                    BinaryOp::Gt => {
                        if lt == SemanticType::F64 {
                            "fcmp ogt"
                        } else {
                            "icmp sgt"
                        }
                    }
                    BinaryOp::GtEq => {
                        if lt == SemanticType::F64 {
                            "fcmp oge"
                        } else {
                            "icmp sge"
                        }
                    }
                    BinaryOp::And => "and",
                    BinaryOp::Or => "or",
                    BinaryOp::Assign => {
                        return Err(CodegenError {
                            message: "assignment is a statement expression only".into(),
                        })
                    }
                };
                let is_compare = matches!(
                    op,
                    BinaryOp::Eq
                        | BinaryOp::NotEq
                        | BinaryOp::Lt
                        | BinaryOp::LtEq
                        | BinaryOp::Gt
                        | BinaryOp::GtEq
                );
                self.output.push_str(&format!(
                    "  {result} = {instruction} {llvm_ty} {lv}, {rv}\n"
                ));
                Ok((result, if is_compare { SemanticType::Bool } else { lt }))
            }
            Expr::Assign { left, right } => {
                let (value, ty) = self.emit_expr(right)?;
                match left.as_ref() {
                    Expr::Unary {
                        op: UnaryOp::Deref,
                        expr,
                    } => {
                        let (pointer, ref_ty) = self.emit_expr(expr)?;
                        let SemanticType::Reference { inner, mutable } = ref_ty else {
                            return Err(CodegenError {
                                message: "assignment target is not a reference".into(),
                            });
                        };
                        if !mutable {
                            return Err(CodegenError {
                                message: "cannot assign through a shared reference".into(),
                            });
                        }
                        let llvm_ty = Self::llvm_type(&inner)?;
                        self.output
                            .push_str(&format!("  store {llvm_ty} {value}, ptr {pointer}\n"));
                        Ok((value, ty))
                    }
                    Expr::Name(name) => {
                        let local = self.locals.get(name).cloned().ok_or_else(|| CodegenError {
                            message: format!("no local storage for `{name}`"),
                        })?;
                        self.output.push_str(&format!(
                            "  store {} {value}, ptr {}\n",
                            Self::llvm_type(&local.ty)?,
                            local.ptr
                        ));
                        Ok((value, ty))
                    }
                    Expr::Index { base, index } => {
                        let base_name = match base.as_ref() {
                            Expr::Name(name) => name,
                            _ => {
                                return Err(CodegenError {
                                    message: "array assignment requires a named local".into(),
                                })
                            }
                        };
                        let local =
                            self.locals
                                .get(base_name)
                                .cloned()
                                .ok_or_else(|| CodegenError {
                                    message: format!("no local storage for `{base_name}`"),
                                })?;
                        let (element_type, length) = match local.ty.clone() {
                            SemanticType::Array(element, length) => (*element, length),
                            _ => {
                                return Err(CodegenError {
                                    message: "array assignment requires an array local".into(),
                                })
                            }
                        };
                        if element_type != ty {
                            return Err(CodegenError {
                                message: "array assignment type mismatch".into(),
                            });
                        }
                        let (index_value, index_type) = self.emit_expr(index)?;
                        if index_type != SemanticType::I32 {
                            return Err(CodegenError {
                                message: "array index must be i32".into(),
                            });
                        }
                        self.emit_bounds_check(&index_value, length);
                        let element_llvm = Self::llvm_type(&element_type)?;
                        let address = self.next_temp();
                        self.output.push_str(&format!("  {address} = getelementptr inbounds [{} x {}], ptr {}, i64 0, i32 {index_value}\n  store {} {value}, ptr {address}\n", length, element_llvm, local.ptr, element_llvm));
                        Ok((value, ty))
                    }
                    Expr::Field { base, name } => {
                        let base_name = match base.as_ref() {
                            Expr::Name(name) => name,
                            _ => {
                                return Err(CodegenError {
                                    message: "field assignment requires a named local".into(),
                                })
                            }
                        };
                        let local =
                            self.locals
                                .get(base_name)
                                .cloned()
                                .ok_or_else(|| CodegenError {
                                    message: format!("no local storage for `{base_name}`"),
                                })?;
                        let struct_name = match local.ty.clone() {
                            SemanticType::Struct(name) => name,
                            _ => {
                                return Err(CodegenError {
                                    message: "field assignment requires a struct local".into(),
                                })
                            }
                        };
                        let fields =
                            self.struct_fields
                                .get(&struct_name)
                                .cloned()
                                .ok_or_else(|| CodegenError {
                                    message: format!("unknown struct `{struct_name}`"),
                                })?;
                        let (field_index, field_type) = fields
                            .iter()
                            .enumerate()
                            .find(|(_, (field_name, _))| field_name == name)
                            .map(|(index, (_, ty))| (index, ty.clone()))
                            .ok_or_else(|| CodegenError {
                                message: format!("unknown field `{name}`"),
                            })?;
                        if field_type != ty {
                            return Err(CodegenError {
                                message: "field assignment type mismatch".into(),
                            });
                        }
                        let struct_llvm = Self::llvm_type(&local.ty)?;
                        let field_llvm = Self::llvm_type(&field_type)?;
                        let address = self.next_temp();
                        self.output.push_str(&format!("  {address} = getelementptr inbounds {struct_llvm}, ptr {}, i32 0, i32 {field_index}\n  store {} {value}, ptr {address}\n", local.ptr, field_llvm));
                        Ok((value, ty))
                    }
                    _ => Err(CodegenError {
                        message: "LLVM assignment target is not a local name".into(),
                    }),
                }
            }
            Expr::Call { callee, args } => {
                let name = match callee.as_ref() {
                    Expr::Name(name) => name,
                    _ => {
                        return Err(CodegenError {
                            message: "LLVM calls require named functions".into(),
                        })
                    }
                };
                let mut rendered = Vec::new();
                for arg in args {
                    let (value, ty) = self.emit_expr(arg)?;
                    rendered.push(format!("{} {value}", Self::llvm_type(&ty)?));
                }
                if name == "print" {
                    if args.len() != 1 {
                        return Err(CodegenError {
                            message: "print expects one String argument".into(),
                        });
                    }
                    self.output
                        .push_str(&format!("  call i32 @puts({})\n", rendered.join(", ")));
                    return Ok(("0".into(), SemanticType::Unit));
                }
                let signature_return = self
                    .function_returns
                    .get(name)
                    .cloned()
                    .unwrap_or(SemanticType::I32);
                let result_ty = Self::llvm_type(&signature_return)?;
                if signature_return == SemanticType::Unit {
                    self.output
                        .push_str(&format!("  call void @{name}({})\n", rendered.join(", ")));
                    return Ok(("0".into(), SemanticType::Unit));
                }
                let result = self.next_temp();
                self.output.push_str(&format!(
                    "  {result} = call {result_ty} @{name}({})\n",
                    rendered.join(", ")
                ));
                Ok((result, signature_return))
            }
            Expr::Array(_) => Err(CodegenError {
                message: "array literals are lowered only in typed local initializers".into(),
            }),
            Expr::Index { base, index } => {
                let name = match base.as_ref() {
                    Expr::Name(name) => name,
                    _ => {
                        return Err(CodegenError {
                            message: "array indexing requires a named local".into(),
                        })
                    }
                };
                let local = self.locals.get(name).cloned().ok_or_else(|| CodegenError {
                    message: format!("no local storage for `{name}`"),
                })?;
                let (index_value, index_type) = self.emit_expr(index)?;
                if index_type != SemanticType::I32 {
                    return Err(CodegenError {
                        message: "array index must be i32".into(),
                    });
                }
                let (element_type, length) = match local.ty.clone() {
                    SemanticType::Array(element, length) => (*element, length),
                    _ => {
                        return Err(CodegenError {
                            message: "indexing requires an array local".into(),
                        })
                    }
                };
                self.emit_bounds_check(&index_value, length);
                let element_llvm = Self::llvm_type(&element_type)?;
                let address = self.next_temp();
                self.output.push_str(&format!("  {address} = getelementptr inbounds [{} x {}], ptr {}, i64 0, i32 {index_value}\n", length, element_llvm, local.ptr));
                let value = self.next_temp();
                self.output.push_str(&format!(
                    "  {value} = load {}, ptr {address}\n",
                    element_llvm
                ));
                Ok((value, element_type))
            }
            Expr::Field { base, name } => {
                let base_name = match base.as_ref() {
                    Expr::Name(name) => name,
                    _ => {
                        return Err(CodegenError {
                            message: "field access requires a named struct local".into(),
                        })
                    }
                };
                let local = self
                    .locals
                    .get(base_name)
                    .cloned()
                    .ok_or_else(|| CodegenError {
                        message: format!("no local storage for `{base_name}`"),
                    })?;
                let struct_name = match local.ty.clone() {
                    SemanticType::Struct(name) => name,
                    _ => {
                        return Err(CodegenError {
                            message: "field access requires a struct local".into(),
                        })
                    }
                };
                let fields = self
                    .struct_fields
                    .get(&struct_name)
                    .cloned()
                    .ok_or_else(|| CodegenError {
                        message: format!("unknown struct `{struct_name}`"),
                    })?;
                let (field_index, field_type) = fields
                    .iter()
                    .enumerate()
                    .find(|(_, (field_name, _))| field_name == name)
                    .map(|(index, (_, ty))| (index, ty.clone()))
                    .ok_or_else(|| CodegenError {
                        message: format!("unknown field `{name}`"),
                    })?;
                let struct_llvm = Self::llvm_type(&local.ty)?;
                let field_llvm = Self::llvm_type(&field_type)?;
                let address = self.next_temp();
                self.output.push_str(&format!("  {address} = getelementptr inbounds {struct_llvm}, ptr {}, i32 0, i32 {field_index}\n", local.ptr));
                let value = self.next_temp();
                self.output
                    .push_str(&format!("  {value} = load {}, ptr {address}\n", field_llvm));
                Ok((value, field_type))
            }
        }
    }
    fn emit_block(&mut self, block: &Block) -> Result<bool, CodegenError> {
        let mut terminated = false;
        for stmt in &block.statements {
            if terminated {
                break;
            }
            terminated = self.emit_stmt(stmt)?;
        }
        Ok(terminated)
    }
    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<bool, CodegenError> {
        match stmt {
            Stmt::Let {
                name, ty, value, ..
            } => {
                let semantic_ty = ty
                    .as_ref()
                    .map(Self::ast_type)
                    .or_else(|| {
                        value.as_ref().map(|e| match e {
                            Expr::Float(_) => SemanticType::F64,
                            Expr::Bool(_) => SemanticType::Bool,
                            _ => SemanticType::I32,
                        })
                    })
                    .unwrap_or(SemanticType::I32);
                let llvm_ty = Self::llvm_type(&semantic_ty)?;
                let ptr = self.next_temp();
                self.output
                    .push_str(&format!("  {ptr} = alloca {llvm_ty}\n"));
                self.locals.insert(
                    name.clone(),
                    Local {
                        ptr: ptr.clone(),
                        ty: semantic_ty.clone(),
                    },
                );
                if let Some(value) = value {
                    if let (SemanticType::Array(element_type, length), Expr::Array(items)) =
                        (&semantic_ty, value)
                    {
                        if *length != items.len() {
                            return Err(CodegenError {
                                message: format!("array initializer for `{name}` has wrong length"),
                            });
                        }
                        let element_llvm = Self::llvm_type(element_type)?;
                        for (index, item) in items.iter().enumerate() {
                            let (rendered, value_ty) = self.emit_expr(item)?;
                            if value_ty != **element_type {
                                return Err(CodegenError {
                                    message: format!(
                                        "array initializer element type mismatch for `{name}`"
                                    ),
                                });
                            }
                            let address = self.next_temp();
                            self.output.push_str(&format!("  {address} = getelementptr inbounds [{} x {}], ptr {}, i64 0, i32 {}\n  store {} {}, ptr {address}\n", length, element_llvm, ptr, index, element_llvm, rendered));
                        }
                    } else {
                        let (rendered, value_ty) = self.emit_expr(value)?;
                        if value_ty != semantic_ty {
                            return Err(CodegenError {
                                message: format!("initializer type mismatch for `{name}`"),
                            });
                        }
                        self.output
                            .push_str(&format!("  store {llvm_ty} {rendered}, ptr {ptr}\n"));
                    }
                }
                Ok(false)
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr)?;
                Ok(false)
            }
            Stmt::Return(expr) => {
                match expr {
                    Some(expr) => {
                        let (value, ty) = self.emit_expr(expr)?;
                        self.output
                            .push_str(&format!("  ret {} {value}\n", Self::llvm_type(&ty)?));
                    }
                    None => self.output.push_str("  ret void\n"),
                }
                Ok(true)
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let (condition_value, condition_type) = self.emit_expr(condition)?;
                if condition_type != SemanticType::Bool {
                    return Err(CodegenError {
                        message: "if condition must lower to i1".into(),
                    });
                }
                let then_label = self.next_label("if.then");
                let else_label = self.next_label("if.else");
                let end_label = self.next_label("if.end");
                self.output.push_str(&format!("  br i1 {condition_value}, label %{then_label}, label %{else_label}\n\n{then_label}:\n"));
                let then_terminated = self.emit_block(then_block)?;
                if !then_terminated {
                    self.output.push_str(&format!("  br label %{end_label}\n"));
                }
                self.output.push_str(&format!("\n{else_label}:\n"));
                let else_terminated = if let Some(block) = else_block {
                    self.emit_block(block)?
                } else {
                    false
                };
                if !else_terminated {
                    self.output.push_str(&format!("  br label %{end_label}\n"));
                }
                self.output.push_str(&format!("\n{end_label}:\n"));
                if then_terminated && else_terminated {
                    self.output.push_str("  unreachable\n");
                }
                Ok(then_terminated && else_terminated)
            }
            Stmt::While { condition, body } => {
                let condition_label = self.next_label("while.cond");
                let body_label = self.next_label("while.body");
                let end_label = self.next_label("while.end");
                self.output.push_str(&format!(
                    "  br label %{condition_label}\n\n{condition_label}:\n"
                ));
                let (condition_value, condition_type) = self.emit_expr(condition)?;
                if condition_type != SemanticType::Bool {
                    return Err(CodegenError {
                        message: "while condition must lower to i1".into(),
                    });
                }
                self.output.push_str(&format!("  br i1 {condition_value}, label %{body_label}, label %{end_label}\n\n{body_label}:\n"));
                self.break_labels.push(end_label.clone());
                self.continue_labels.push(condition_label.clone());
                let body_terminated = self.emit_block(body)?;
                self.break_labels.pop();
                self.continue_labels.pop();
                if !body_terminated {
                    self.output
                        .push_str(&format!("  br label %{condition_label}\n"));
                }
                self.output.push_str(&format!("\n{end_label}:\n"));
                Ok(false)
            }
            Stmt::For {
                name,
                start,
                end,
                body,
            } => {
                let (start_value, start_type) = self.emit_expr(start)?;
                if start_type != SemanticType::I32 {
                    return Err(CodegenError {
                        message: "for-range start must be i32".into(),
                    });
                }
                let (end_value, end_type) = self.emit_expr(end)?;
                if end_type != SemanticType::I32 {
                    return Err(CodegenError {
                        message: "for-range end must be i32".into(),
                    });
                }
                let iterator_ptr = self.next_temp();
                self.output.push_str(&format!("  {iterator_ptr} = alloca i32\n  store i32 {start_value}, ptr {iterator_ptr}\n"));
                self.locals.insert(
                    name.clone(),
                    Local {
                        ptr: iterator_ptr.clone(),
                        ty: SemanticType::I32,
                    },
                );
                let condition_label = self.next_label("for.cond");
                let body_label = self.next_label("for.body");
                let increment_label = self.next_label("for.inc");
                let end_label = self.next_label("for.end");
                self.output.push_str(&format!(
                    "  br label %{condition_label}\n\n{condition_label}:\n"
                ));
                let current = self.next_temp();
                let condition = self.next_temp();
                self.output.push_str(&format!("  {current} = load i32, ptr {iterator_ptr}\n  {condition} = icmp slt i32 {current}, {end_value}\n  br i1 {condition}, label %{body_label}, label %{end_label}\n\n{body_label}:\n"));
                self.break_labels.push(end_label.clone());
                self.continue_labels.push(increment_label.clone());
                let body_terminated = self.emit_block(body)?;
                self.break_labels.pop();
                self.continue_labels.pop();
                if !body_terminated {
                    self.output
                        .push_str(&format!("  br label %{increment_label}\n"));
                }
                self.output.push_str(&format!("\n{increment_label}:\n"));
                if body_terminated {
                    self.output.push_str("  unreachable\n");
                } else {
                    let next = self.next_temp();
                    self.output.push_str(&format!("  {next} = add i32 {current}, 1\n  store i32 {next}, ptr {iterator_ptr}\n  br label %{condition_label}\n"));
                }
                self.output.push_str(&format!("\n{end_label}:\n"));
                Ok(false)
            }
            Stmt::Break => {
                let label = self
                    .break_labels
                    .last()
                    .cloned()
                    .ok_or_else(|| CodegenError {
                        message: "break outside loop".into(),
                    })?;
                self.output.push_str(&format!("  br label %{label}\n"));
                Ok(true)
            }
            Stmt::Continue => {
                let label = self
                    .continue_labels
                    .last()
                    .cloned()
                    .ok_or_else(|| CodegenError {
                        message: "continue outside loop".into(),
                    })?;
                self.output.push_str(&format!("  br label %{label}\n"));
                Ok(true)
            }
        }
    }
    fn emit_function(&mut self, function: &Function) -> Result<(), CodegenError> {
        self.temp = 0;
        self.locals.clear();
        self.current_return = function
            .return_type
            .as_ref()
            .map(Self::ast_type)
            .unwrap_or(SemanticType::Unit);
        let return_ty = Self::llvm_type(&self.current_return)?;
        let mut params = Vec::new();
        for (name, ty) in &function.params {
            let semantic_ty = Self::ast_type(ty);
            params.push(format!("{} %arg_{}", Self::llvm_type(&semantic_ty)?, name));
        }
        self.output.push_str(&format!(
            "define {} @{}({}) {{\nentry:\n",
            return_ty,
            function.name,
            params.join(", ")
        ));
        for (name, ty) in &function.params {
            let semantic_ty = Self::ast_type(ty);
            let llvm_ty = Self::llvm_type(&semantic_ty)?;
            let (ptr, local_ty) = if let SemanticType::Reference { .. } = &semantic_ty {
                (format!("%arg_{name}"), semantic_ty.clone())
            } else {
                let ptr = self.next_temp();
                self.output.push_str(&format!(
                    "  {ptr} = alloca {llvm_ty}\n  store {llvm_ty} %arg_{name}, ptr {ptr}\n"
                ));
                (ptr, semantic_ty.clone())
            };
            self.locals
                .insert(name.clone(), Local { ptr, ty: local_ty });
        }
        let mut terminated = false;
        for stmt in &function.body.statements {
            if terminated {
                break;
            }
            terminated = self.emit_stmt(stmt)?;
        }
        if !terminated {
            if self.current_return == SemanticType::Unit {
                self.output.push_str("  ret void\n");
            } else {
                return Err(CodegenError {
                    message: format!(
                        "function `{}` may not return a value on every path",
                        function.name
                    ),
                });
            }
        }
        self.output.push_str("}\n\n");
        Ok(())
    }
}

pub fn generate_llvm(program: &Program) -> Result<String, CodegenError> {
    let mut codegen = LlvmCodegen::new();
    codegen
        .output
        .push_str("; ModuleID = 'vak'\nsource_filename = \"vak\"\n\ndeclare i32 @puts(ptr)\ndeclare void @llvm.trap()\n\n");
    for item in &program.items {
        if let Item::Struct(decl) = item {
            let fields: Vec<(String, SemanticType)> = decl
                .fields
                .iter()
                .map(|(name, ty)| (name.clone(), LlvmCodegen::ast_type(ty)))
                .collect();
            let rendered: Vec<String> = fields
                .iter()
                .map(|(_, ty)| LlvmCodegen::llvm_type(ty))
                .collect::<Result<_, _>>()?;
            codegen.output.push_str(&format!(
                "%struct.{} = type {{ {} }}\n",
                decl.name,
                rendered.join(", ")
            ));
            codegen.struct_fields.insert(decl.name.clone(), fields);
        }
    }
    if program
        .items
        .iter()
        .any(|item| matches!(item, Item::Struct(_)))
    {
        codegen.output.push('\n');
    }
    for item in &program.items {
        if let Item::Function(function) = item {
            let return_type = function
                .return_type
                .as_ref()
                .map(LlvmCodegen::ast_type)
                .unwrap_or(SemanticType::Unit);
            codegen
                .function_returns
                .insert(function.name.clone(), return_type);
        }
    }
    for item in &program.items {
        if let Item::Function(function) = item {
            codegen.emit_function(function)?;
        }
    }
    if codegen.globals.is_empty() {
        return Ok(codegen.output);
    }
    if let Some(position) = codegen.output.find("define ") {
        codegen
            .output
            .insert_str(position, &format!("{}\n", codegen.globals));
    } else {
        codegen.output.push_str(&codegen.globals);
    }
    Ok(codegen.output)
}

#[cfg(test)]
mod codegen_tests {
    use super::*;
    #[test]
    fn emits_valid_shape_for_scalar_function() {
        let program = parse_source("fn add(a: I32, b: I32) -> I32 { return a + b; }").unwrap();
        analyze(&program).unwrap();
        let ir = generate_llvm(&program).unwrap();
        assert!(ir.contains("define i32 @add"));
        assert!(ir.contains("add i32"));
        assert!(ir.contains("ret i32"));
    }
    #[test]
    fn emits_string_runtime_call() {
        let program = parse_source("fn main() { print(\"hello\"); }").unwrap();
        analyze(&program).unwrap();
        let ir = generate_llvm(&program).unwrap();
        assert!(ir.contains("@.str.0"));
        assert!(ir.contains("call i32 @puts"));
    }
}
