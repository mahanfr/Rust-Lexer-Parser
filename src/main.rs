use std::{fs::{File, self}, io::{BufReader, Read}, process::exit, fmt::Display};


#[derive(Debug)]
enum TToken {
    Identifier,
    Number,
    StringLiteral,
    CharLiteral,
    DOLLAR,
    OPAREN,
    CPAREN,
    OCURLY,
    CCURLY,
    OBRACE,
    CBRACE,
    COMMA,
    COLON,
    SEMICOLON,
    MINUS,
    PLUS,
    EQ,
    MULTY,
    DEVIDE,
    MOD,
    NOT,
    DOT,
    SMALLER,
    BIGGER,
    AND,
    OR,
    IF,
    ELSE,
    FOR,
    WHILE,
    LOOP,
    BREAK,
    CONTINUE,
    RETURN,
    INCLUDE,
    TO,
    IN,
    ENUM,
    STRUCT,
}

impl TToken {
    pub fn is_single_char_token(char: u8) -> Option<TToken> {
        match char {
            b'{' => {Some(TToken::OCURLY)},
            b'}' => {Some(TToken::CCURLY)},
            b'[' => {Some(TToken::OBRACE)},
            b']' => {Some(TToken::CBRACE)},
            b'(' => {Some(TToken::OPAREN)},
            b')' => {Some(TToken::CPAREN)},
            b',' => {Some(TToken::COMMA)},
            b'.' => {Some(TToken::DOT)},
            b';' => {Some(TToken::SEMICOLON)},
            b'-' => {Some(TToken::MINUS)},
            b'+' => {Some(TToken::PLUS)},
            b'*' => {Some(TToken::MULTY)},
            b'/' => {Some(TToken::DEVIDE)},
            b'%' => {Some(TToken::MOD)},
            b'!' => {Some(TToken::NOT)},
            b'$' => {Some(TToken::DOLLAR)},
            b':' => {Some(TToken::COLON)},
            b'=' => {Some(TToken::EQ)},
            b'<' => {Some(TToken::SMALLER)},
            b'>' => {Some(TToken::BIGGER)},
            b'&' => {Some(TToken::AND)},
            b'|' => {Some(TToken::OR)},
            _ => {None}
        }
    }

}

type Loc = (String,usize,usize);

#[allow(dead_code)]
#[derive(Debug)]
struct Lexer {
    file_path: String,
    source: Vec<u8>,
    cur: usize,
    bol: usize,
    row: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    ttype: TToken,
    literal: Vec<u8>,
    file_path : String,
    col: usize,
    line: usize,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Token ({:?}) \"{}\" {}:{}:{}",
            self.ttype,String::from_utf8_lossy(self.literal.as_slice()),
            self.file_path,
            self.line,
            self.col
        )
    }

}
impl Token {
    pub fn new(ttype: TToken, literal: Vec<u8>,loc : Loc) -> Self {
        Self {ttype, literal, file_path: loc.0, line: loc.1, col: loc.2}
    }
}

impl Lexer {
    pub fn new(file_path: impl ToString) -> Self {
        let buf = fs::read(file_path.to_string()).unwrap();
        Self { file_path: file_path.to_string(), source: buf , cur: 0, bol: 0, row: 0 }
    }

    fn drop_char(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == b'\n'{
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn drop_line(&mut self) {
        while !self.is_empty() && self.source[self.cur] != b'\n' {
            self.drop_char();
        }
        if !self.is_empty() {
            self.drop_char();
        }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_ascii_whitespace() {
            self.drop_char()
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_left();
        while !self.is_empty() {
            let sub = self.source[self.cur..self.cur+1].to_vec();
            if sub != b"//" {break;}
            self.drop_line();
            self.trim_left();
        }
        let loc = (self.file_path.to_string(), self.row + 1, self.cur - self.bol + 1);
        if self.is_empty() {return None;}
        
        let first = self.source[self.cur];

        if first.is_ascii_alphabetic() || first == b'_' {
            let index = self.cur;
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphabetic() || self.source[self.cur] == b'_') {
                    self.drop_char();
            }
            let literal = self.source[index..self.cur].to_vec();
            match literal.as_slice() {
                b"if" => {return Some(Token::new(TToken::IF,literal,loc));}
                b"else" => {return Some(Token::new(TToken::ELSE,literal,loc));}
                b"for" => {return Some(Token::new(TToken::FOR,literal,loc));}
                b"while" => {return Some(Token::new(TToken::WHILE,literal,loc));}
                b"loop" => {return Some(Token::new(TToken::LOOP,literal,loc));}
                b"break" => {return Some(Token::new(TToken::BREAK,literal,loc));}
                b"continue" => {return Some(Token::new(TToken::CONTINUE,literal,loc));}
                b"return" => {return Some(Token::new(TToken::RETURN,literal,loc));}
                b"include" => {return Some(Token::new(TToken::INCLUDE,literal,loc));}
                b"to" => {return Some(Token::new(TToken::TO,literal,loc));}
                b"in" => {return Some(Token::new(TToken::IN,literal,loc));}
                b"enum" => {return Some(Token::new(TToken::ENUM,literal,loc));}
                b"struct" => {return Some(Token::new(TToken::STRUCT,literal,loc));}
                _ => {
                    return Some(Token::new(TToken::Identifier,literal,loc));
                }
            }
        }

        if first.is_ascii_digit() {
            let index = self.cur;
            while !self.is_empty() && self.source[self.cur].is_ascii_digit() {
                self.drop_char();
            }
            let literal = self.source[index..self.cur].to_vec();
            return Some(Token::new(TToken::Number,literal,loc));
        }
        
        if first == b'\'' {
            self.drop_char();
            let mut literal = Vec::<u8>::new();
            let char = self.source[self.cur];
            if char == b'\'' {
                println!("char literal can not be empty :{}:{}:{}",loc.0,loc.1,loc.2);
                exit(1);
            }
            if char == b'\\' {
                self.drop_char();
                if self.is_empty() {
                    println!("char literal unfinished escape sequence :{}:{}:{}",loc.0,loc.1,loc.2);
                    exit(1);
                }
                let escape = self.source[self.cur];
                match escape {
                    b'n' => {
                        literal.push(b'\n');
                        self.drop_char();
                    },
                    b'\'' => {
                        literal.push(b'\'');
                        self.drop_char();
                    },
                    b't' => {
                        literal.push(b'\t');
                        self.drop_char();
                    },
                    b'r' => {
                        literal.push(b'\r');
                        self.drop_char();
                    },
                    b'\\' => {
                        literal.push(b'\\');
                        self.drop_char();
                    },
                    _ => {
                        println!("unsupported escape sequence (\\{}) :{}:{}:{}",escape,loc.0,loc.1,loc.2);
                        exit(1);
                    }
                }
            }else{
                literal.push(char);
                self.drop_char();
            }

            if !self.is_empty() {
                if self.source[self.cur] != b'\'' {
                    println!("unsupported char :{}:{}:{}",loc.0,loc.1,loc.2);
                    exit(1);
                }
                self.drop_char();
                return Some(Token::new(TToken::CharLiteral,literal,loc));
            }
        }

        if first == b'"' {
            self.drop_char();
            let mut literal = Vec::<u8>::new();
            while !self.is_empty() {
                let char = self.source[self.cur];
                if char == b'"' {break;}
                if char == b'\n' {
                    println!("string literal not closed before end of line :{}:{}:{}",loc.0,loc.1,loc.2);
                    exit(1);
                }
                if char == b'\\' {
                    self.drop_char();
                    if self.is_empty() {
                        println!("string literal unfinished escape sequence :{}:{}:{}",loc.0,loc.1,loc.2);
                        exit(1);
                    }

                    let escape = self.source[self.cur];
                    match escape {
                        b'n' => {
                            literal.push(b'\n');
                            self.drop_char();
                        },
                        b'"' => {
                            literal.push(b'"');
                            self.drop_char();
                        },
                        b't' => {
                            literal.push(b'\t');
                            self.drop_char();
                        },
                        b'r' => {
                            literal.push(b'\r');
                            self.drop_char();
                        },
                        b'\\' => {
                            literal.push(b'\\');
                            self.drop_char();
                        },
                        _ => {
                            println!("unsupported escape sequence (\\{}) :{}:{}:{}",escape,loc.0,loc.1,loc.2);
                            exit(1);
                        }
                    }
                }
                literal.push(char);
                self.drop_char();
            }
            if !self.is_empty() {
                self.drop_char();
                return Some(Token::new(TToken::StringLiteral,literal,loc));
            }
        }

        match TToken::is_single_char_token(first) {
            Some(tt) => {
                self.drop_char();
                return Some(Token::new(tt,[first].to_vec(),loc));
            },
            None => ()
        }

        println!("string literal not closed before EOF :{}:{}:{}",loc.0,loc.1,loc.2);
        exit(1);
    }
}

fn main() {
    let mut lexer = Lexer::new("grammer.txt");
    loop {
        let token = lexer.next_token();
        if token.is_none() {break;}
        println!("{}",token.unwrap());
    }
}
