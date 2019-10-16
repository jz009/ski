use regex;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Literal {
    Str(String),
    Int(u64),
    Bool(bool),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum LiteralType {
    Str,
    Int,
    None
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Keyword {
    Function,
    Let,
    Const,
    For,
    In,
    While,
    Loop,
    Return,
    If,
    Else
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Symbol {
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    SemiColon,
    DoubleEqual,
    Equal,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    SingleQuote,
    DoubleQuote,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Shr,
    Shl,
    Xor,
    LogicalAnd,
    LogicalOr,
    BinaryAnd,
    BinaryOr,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Literal(Literal),
    Keyword(Keyword),
    Symbol(Symbol)
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Pos {
    row: u16,
    col: u16
}

impl Pos {
    pub fn new() -> Pos {
        Pos {
            row: 0,
            col: 0
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    pos: Pos,
}

impl TokenKind {
    pub fn new(token: &str) -> TokenKind {
        match token {
            "fn" => TokenKind::Keyword(Keyword::Function),
            "let" => TokenKind::Keyword(Keyword::Let),
            "const" => TokenKind::Keyword(Keyword::Const),
            "for" => TokenKind::Keyword(Keyword::For),
            "while" => TokenKind::Keyword(Keyword::While),
            "loop" => TokenKind::Keyword(Keyword::Loop),
            "return" => TokenKind::Keyword(Keyword::Return),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "in" => TokenKind::Keyword(Keyword::In),
            "{" => TokenKind::Symbol(Symbol::OpenBracket),
            "}" => TokenKind::Symbol(Symbol::CloseBracket),
            "(" => TokenKind::Symbol(Symbol::OpenParen),
            ")" => TokenKind::Symbol(Symbol::CloseParen),
            "=" => TokenKind::Symbol(Symbol::Equal),
            "==" => TokenKind::Symbol(Symbol::DoubleEqual),
            ";" => TokenKind::Symbol(Symbol::SemiColon),
            "'" => TokenKind::Symbol(Symbol::SingleQuote),
            "\"" => TokenKind::Symbol(Symbol::DoubleQuote),
            "+" => TokenKind::Symbol(Symbol::Add),
            "-" => TokenKind::Symbol(Symbol::Sub),
            "*" => TokenKind::Symbol(Symbol::Mul),
            "/" => TokenKind::Symbol(Symbol::Div),
            "**" => TokenKind::Symbol(Symbol::Pow),
            "+=" => TokenKind::Symbol(Symbol::AddAssign),
            "-=" => TokenKind::Symbol(Symbol::SubAssign),
            ">" => TokenKind::Symbol(Symbol::Gt),
            "<" => TokenKind::Symbol(Symbol::Lt),
            ">=" => TokenKind::Symbol(Symbol::GtEq),
            "<=" => TokenKind::Symbol(Symbol::LtEq),
            ">>" => TokenKind::Symbol(Symbol::Shr),
            "<<" => TokenKind::Symbol(Symbol::Shl),
            "^" => TokenKind::Symbol(Symbol::Xor),
            "&" => TokenKind::Symbol(Symbol::BinaryAnd),
            "|" => TokenKind::Symbol(Symbol::BinaryOr),
            "&&" => TokenKind::Symbol(Symbol::LogicalAnd),
            "||" => TokenKind::Symbol(Symbol::LogicalOr),
            "true" => TokenKind::Literal(Literal::Bool(true)),
            "false" => TokenKind::Literal(Literal::Bool(false)),
            _ => {
                TokenKind::Identifier(token.to_owned())
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Token {
    token_kind: TokenKind,
    pos: Pos
}

impl Token {
    pub fn new(token_kind: TokenKind, pos: Pos) -> Token {
        Token {
            token_kind,
            pos
        }
    }
}

macro_rules! double_identifier {
    ( $i:literal, $cur:ident, $idents:ident, $self:ident ) => {
        if $cur == $i {
            $idents.push(Token {
                    token_kind: TokenKind::new(concat!($i, $i)),
                    pos: $self.pos
                }
            );
            $cur = "";
            continue;
        }
        
        if $cur != "" {
            $idents.push(Token {
                    token_kind: TokenKind::new($cur),
                    pos: $self.pos
                });
        }

        $cur = $i;
    }
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            pos: Pos::new()
        }
    }

    pub fn strip_comments(input: &str) -> Result<String, regex::Error> {
        let single_line = regex::Regex::new(r"//[^\n\r]*")?;
        let multi_line = regex::Regex::new(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")?;

        Ok(
            single_line.replace_all(
                multi_line.replace_all(input, "").to_mut(), ""
            ).to_mut().to_string())
    }

    pub fn lex(&mut self, s: &str) -> Result<Vec<Token>, regex::Error> {
        let input = Lexer::strip_comments(s)?;
        let mut tokens: Vec<Token> = Vec::with_capacity(40);

        let mut ci: String;

        let mut current_identifier: &str = "";

        let mut literal = LiteralType::None;

        let mut integer_base: u32 = 10;
        // let mut next_char_is_escaped = false;

        for c in input.chars() {
            self.pos.col += 1;
            match literal {
                LiteralType::Str => {
                    if c == '"' {
                        tokens.push(Token {
                                token_kind: TokenKind::Literal(Literal::Str(current_identifier.to_owned())),
                                pos: self.pos
                            }
                        );
                        current_identifier = "";
                        literal = LiteralType::None;
                        continue;
                    }

                    ci = format!("{}{}", current_identifier, c);
                    current_identifier = ci.as_ref();
                    continue;
                },
                LiteralType::Int => {
                    match c {
                        '0'..='9' => {
                            ci = format!("{}{}", current_identifier, c);
                            current_identifier = ci.as_ref();
                            continue;
                        },
                        'a'..='f' | 'A'..='F' => {
                            if integer_base == 16 {
                                ci = format!("{}{}", current_identifier, c);
                                current_identifier = ci.as_ref();
                            } else {
                                unimplemented!()
                            }
                            continue;
                        }
                        'x' => {
                            if current_identifier.len() == 1 {
                                current_identifier = "";
                                integer_base = 16;
                            } else {
                                unimplemented!()
                            }
                            continue;
                        },
                        _ => {
                            tokens.push(Token {
                                token_kind: TokenKind::Literal(Literal::Int(u64::from_str_radix(current_identifier, integer_base).unwrap())),
                                pos: self.pos
                            });
                            integer_base = 10;
                            current_identifier = "";
                            literal = LiteralType::None;
                        }
                    } 
                },
                _ => {}
            }

            match c {
                ' ' | '\r' | '\t' => {
                    if current_identifier != "" {
                        tokens.push(
                            Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            }
                        );
                        current_identifier = "";
                    }
                },
                '\n' => {
                    if current_identifier != "" {
                        tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                        current_identifier = "";
                    }
                    self.pos.row += 1;
                    self.pos.col = 0;
                }
                '{' | '}' | '(' | ')' | ';' | '^' => {
                    if current_identifier != "" {
                        tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                        current_identifier = "";
                    }
                    tokens.push(Token {
                            token_kind: TokenKind::new(&c.to_string()),
                            pos: self.pos
                        }
                    );
                },
                '+' => {
                    if current_identifier != "" {
                        tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                    }
                    current_identifier = "+";
                },
                '-' => {
                    if current_identifier != "" {
                        tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                    }
                    current_identifier = "-";
                },
                '"' => {
                    literal = LiteralType::Str;
                    continue;
                }
                '=' => {
                    match current_identifier {
                        "=" | "+" | "-" | "*" | "/" | "<" | ">" => {
                            tokens.push(
                                Token {
                                        token_kind: TokenKind::new(&format!("{}{}", current_identifier, "=")),
                                        pos: self.pos
                                    }
                            );
                            current_identifier = "";
                        },
                        _ => {
                            if current_identifier != "" {
                                tokens.push(Token {
                                    token_kind: TokenKind::new(current_identifier),
                                    pos: self.pos
                                });
                            }
                            current_identifier = "=";
                        }
                    }
                    
                }
                '&' => {
                    double_identifier!("&", current_identifier, tokens, self);
                }
                '|' => {
                    double_identifier!("|", current_identifier, tokens, self);
                }
                '*' => {
                    double_identifier!("*", current_identifier, tokens, self);
                }
                '/' => {
                    double_identifier!("/", current_identifier, tokens, self);
                }
                '>' => {
                    double_identifier!(">", current_identifier, tokens, self);
                }
                '<' => {
                    double_identifier!("<", current_identifier, tokens, self);
                }
                '0'..='9' => {
                    literal = LiteralType::Int;
                    if current_identifier != "" {
                        tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                        current_identifier = "";
                    }
                    ci = format!("{}{}", current_identifier, c);
                    current_identifier = ci.as_ref();
                }
                _ => {
                    match current_identifier {
                        "=" | "&" | "|" | "*" | "/" | "<" | ">" | "+" | "-" => {
                            tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                            current_identifier = "";
                        }
                        _ => {}
                    }
                    ci = format!("{}{}", current_identifier, c);
                    current_identifier = ci.as_ref();
                }
            }
        }
        match literal {
            LiteralType::Int => {
                tokens.push(Token {
                        token_kind: TokenKind::Literal(Literal::Int(u64::from_str_radix(current_identifier, integer_base).unwrap())),
                        pos: self.pos
                    }
                );
            }
            LiteralType::Str => {
                tokens.push(Token {
                        token_kind: TokenKind::Literal(Literal::Str(current_identifier.to_owned())),
                        pos: self.pos
                    });
            }
            _ => {
                if current_identifier != "" {
                    tokens.push(Token {
                                token_kind: TokenKind::new(current_identifier),
                                pos: self.pos
                            });
                }
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use TokenKind::*;
    use super::Literal::*;
    use super::Symbol::*;
    #[test]
    fn test() {
        let mut lexer = Lexer::new();
        /*
        assert_eq!(
            lexer.lex("fn hi(){if 1==1{print(\"hi\");}}").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("hi")), Token::Symbol(OpenParen), Token::Symbol(CloseParen), Token::Symbol(OpenBracket), Token::Keyword(super::Keyword::If), Literal(Int(1)), Token::Symbol(DoubleEqual), Literal(Int(1)), Token::Symbol(OpenBracket), Identifier(String::from("print")), Token::Symbol(OpenParen), Literal(Str(String::from("hi"))), Token::Symbol(CloseParen), Token::Symbol(SemiColon), Token::Symbol(CloseBracket), Token::Symbol(CloseBracket))
        );
        assert_eq!(
            lexer.lex("fn hi() {\n\tif 1 == 1 {\n\t\tprint(\"hi\");\n}}").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("hi")), Token::Symbol(OpenParen), Token::Symbol(CloseParen), Token::Symbol(OpenBracket), Token::Keyword(super::Keyword::If), Literal(Int(1)), Token::Symbol(DoubleEqual), Literal(Int(1)), Token::Symbol(OpenBracket), Identifier(String::from("print")), Token::Symbol(OpenParen), Literal(Str(String::from("hi"))), Token::Symbol(CloseParen), Token::Symbol(SemiColon), Token::Symbol(CloseBracket), Token::Symbol(CloseBracket))
        );
        assert_eq!(
            lexer.lex("/**/fn hi/**//**/() {\n\tif 1 == 1 {\n\t\tprint(\"hi\");\n}}// aje=  df d").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("hi")), Token::Symbol(OpenParen), Token::Symbol(CloseParen), Token::Symbol(OpenBracket), Token::Keyword(super::Keyword::If), Literal(Int(1)), Token::Symbol(DoubleEqual), Literal(Int(1)), Token::Symbol(OpenBracket), Identifier(String::from("print")), Token::Symbol(OpenParen), Literal(Str(String::from("hi"))), Token::Symbol(CloseParen), Token::Symbol(SemiColon), Token::Symbol(CloseBracket), Token::Symbol(CloseBracket))
        );
        assert_eq!(
            lexer.lex("1+1").unwrap(),
            vec!(Literal(Int(1)), Token::Symbol(Add), Literal(Int(1)))
        );
        assert_eq!(
            lexer.lex("1+1 ").unwrap(),
            vec!(Literal(Int(1)), Token::Symbol(Add), Literal(Int(1)))
        );
        assert_eq!(
            lexer.lex("1 + 1").unwrap(),
            vec!(Literal(Int(1)), Token::Symbol(Add), Literal(Int(1)))
        );
        assert_eq!(
            lexer.lex("let x = 1 + 1;").unwrap(),
            vec!(Token::Keyword(super::Keyword::Let), Identifier(String::from("x")), Token::Symbol(Equal), Literal(Int(1)), Token::Symbol(Add), Literal(Int(1)), Token::Symbol(SemiColon))
        );
        assert_eq!(
            lexer.lex("\"hi\"").unwrap(),
            vec!(Literal(Str(String::from("hi"))))
        );
        assert_eq!(
            lexer.lex("fn func()").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("func")), Token::Symbol(OpenParen), Token::Symbol(CloseParen))
        );
         assert_eq!(
            lexer.lex("fn func ()").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("func")), Token::Symbol(OpenParen), Token::Symbol(CloseParen))
        );
         assert_eq!(
            lexer.lex("fn func (  )").unwrap(),
            vec!(Token::Keyword(super::Keyword::Function), Identifier(String::from("func")), Token::Symbol(OpenParen), Token::Symbol(CloseParen))
        );
        assert_eq!(
            lexer.lex("| |").unwrap(),
            vec!(BinaryOr, BinaryOr)
        );
        */
      
    }
}