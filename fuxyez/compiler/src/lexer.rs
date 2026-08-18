use std::str::Chars;
use crate::diagnostics::{Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Keyword(String),
    Symbol(char),
    Operator(String),
    Newline,
    Whitespace,
    Comment(String),
    Eof,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    src: &'a str,
    chars: Chars<'a>,
    current_pos: usize,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars(),
            current_pos: 0,
            diagnostics: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn bump(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(ch) = next {
            self.current_pos += ch.len_utf8();
        }
        next
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            let start = self.current_pos;
            match ch {
                c if c.is_ascii_whitespace() => {
                    let mut ws = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_whitespace() && c != '\n' {
                            ws.push(self.bump().unwrap());
                        } else {
                            break;
                        }
                    }
                    return Token { kind: TokenKind::Whitespace, span: Span::new(start, self.current_pos) };
                }
                '\n' => {
                    self.bump();
                    return Token { kind: TokenKind::Newline, span: Span::new(start, self.current_pos) };
                }
                c if c.is_ascii_digit() => {
                    let mut val = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() || c == '.' {
                            val.push(self.bump().unwrap());
                        } else {
                            break;
                        }
                    }
                    return Token { kind: TokenKind::Number(val), span: Span::new(start, self.current_pos) };
                }
                '"' => {
                    self.bump(); // skip opening quote
                    let mut val = String::new();
                    while let Some(c) = self.bump() {
                        if c == '"' {
                            break;
                        } else {
                            val.push(c);
                        }
                    }
                    return Token { kind: TokenKind::StringLiteral(val), span: Span::new(start, self.current_pos) };
                }
                '/' if self.src[start+1..].starts_with('/') => {
                    let mut comment = String::new();
                    self.bump(); self.bump(); // skip //
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        comment.push(self.bump().unwrap());
                    }
                    return Token { kind: TokenKind::Comment(comment), span: Span::new(start, self.current_pos) };
                }
                c if is_symbol(c) => {
                    let sym = self.bump().unwrap();
                    return Token { kind: TokenKind::Symbol(sym), span: Span::new(start, self.current_pos) };
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(self.bump().unwrap());
                        } else {
                            break;
                        }
                    }
                    if is_keyword(&ident) {
                        return Token { kind: TokenKind::Keyword(ident), span: Span::new(start, self.current_pos) };
                    }
                    return Token { kind: TokenKind::Identifier(ident), span: Span::new(start, self.current_pos) };
                }
                _ => {
                    let err = format!("Unexpected char: {}", ch);
                    self.bump();
                    self.diagnostics.push(Diagnostic::error(err.clone(), Span::new(start, self.current_pos)));
                    return Token { kind: TokenKind::Error(err), span: Span::new(start, self.current_pos) };
                }
            }
        }
        Token { kind: TokenKind::Eof, span: Span::new(self.current_pos, self.current_pos) }
    }
}

fn is_symbol(c: char) -> bool {
    "(){}[],;.:".contains(c)
}

fn is_keyword(s: &str) -> bool {
    matches!(s, "let" | "fn" | "if" | "else" | "for" | "while" | "return" | "async" | "yield" | "const" | "sigil" | "on")
}