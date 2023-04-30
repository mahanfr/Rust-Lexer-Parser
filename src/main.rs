#![allow(dead_code)]
mod parser;
use std::{process::exit, vec};

use parser::{Lexer, TToken, Token};

#[derive(Debug)]
struct Program {
    shebang: String,
    body: Vec<Node>,
}

impl Program {
    pub fn new(lexer: &mut Lexer) -> Self {
        let mut body = Vec::<Node>::new();
        loop {
            let token = lexer.next_token();
            if token.ttype == TToken::Fun {
                body.push(Func::get_node(lexer));
            }else if token.ttype == TToken::ATSIGN {
                body.push(VariableDelclear::get_node(lexer));
            }else if token.ttype == TToken::EOF {
                break;
            }else {
                println!("Syntax error unexpected token ({:?}) at {}:{}:{}",token.ttype,token.file_path,token.line,token.col);
                exit(1);
            }
        }
        Self {
            shebang: String::new(),
            body,
        }
    }
}

#[derive(Debug,PartialEq)]
struct Type {
    name: String,
}

#[derive(Debug, PartialEq)]
enum Expr {
    ExprBinary,
    ExprUnary,
}

#[derive(Debug, PartialEq)]
struct ExprPath {}

#[derive(Debug, PartialEq)]
struct ExprAssign {
    left: ExprPath,
    right: Expr,
}

#[derive(Debug,PartialEq)]
struct VariableDelclear {
    is_const: bool,
    is_static: bool,
    ident: String,
    kind: Type,
    // TODO: Change this shit
    init_value: String,
}

// [ident, ident, semicolon]
// [ident, ident, colon, colon, value, semicolon]
// [ident, ident, colon, value, semicolon]
// [ident, ident, equal, value, semicolon]
// [ident, colon, value, semicolon]
// [ident, equal, value, semicolon]
impl VariableDelclear {
    fn new(lexer: &mut Lexer) -> Self {
        let is_const: bool;
        let is_static: bool;
        let ident: String;
        let kind: Type;
        let init_value: String;
        let mut token = expect_token(lexer, vec![TToken::Identifier]);
        ident = token.get_literal_string();
        token = expect_token(lexer, vec![TToken::Identifier,TToken::COLON,TToken::EQ]);
        match token.ttype {
            TToken::Identifier => {
                kind = Type { name: token.get_literal_string() };
                token = expect_token(lexer, vec![TToken::COLON,TToken::EQ,TToken::SEMICOLON]);
                match token.ttype {
                    TToken::COLON => {
                        token = expect_token(lexer, vec![TToken::COLON,TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        if token.ttype == TToken::COLON {
                            is_const = true;
                            is_static = true;
                            token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        }else {
                            is_const = true;
                            is_static = false;
                        }
                        init_value = token.get_literal_string();
                        expect_token(lexer, vec![TToken::SEMICOLON]);
                    },
                    TToken::EQ => {
                        is_const = false;
                        is_static = false;
                        token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        init_value = token.get_literal_string();
                        expect_token(lexer, vec![TToken::SEMICOLON]);
                    },
                    TToken::SEMICOLON => {
                        is_const = false;
                        is_static = false;
                        init_value = String::new();
                    },
                    _ => {unreachable!();}
                }
            },
            TToken::COLON => {
                kind = Type { name: "undifiend".to_string() };
                is_const = true;
                is_static = false;
                token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                init_value = token.get_literal_string();
                expect_token(lexer, vec![TToken::SEMICOLON]);
            },
            TToken::EQ => {
                kind = Type { name: "undifiend".to_string() };
                is_const = false;
                is_static = false;
                token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                init_value = token.get_literal_string();
                expect_token(lexer, vec![TToken::SEMICOLON]);
            },
            _ => {unreachable!();},
        }
        return Self { is_const, is_static, ident, kind, init_value };
    }

    fn get_node(lexer: &mut Lexer) -> Node {
        Node::VariableDelclear { var: Self::new(lexer) }
    }
}

#[derive(Debug)]
struct Arg {
    ident: String,
    kind: Type,
}

#[derive(Debug)]
struct Func {
    ident: String,
    args: Vec<Arg>,
    return_type: Type,
    block: Vec<Node>,
}
impl Func {
    pub fn new(lexer: &mut Lexer) -> Self {
        let token = expect_token(lexer, vec![TToken::Identifier]);
        let ident = String::from_utf8(token.literal).unwrap();
        expect_token(lexer, vec![TToken::OPAREN]);
        let mut args = Vec::<Arg>::new();
        loop{
            let token = expect_token(lexer, vec![TToken::Identifier,TToken::CPAREN,TToken::COMMA]);
            if token.ttype == TToken::Identifier {
                let type_token = expect_token(lexer, vec![TToken::Identifier]);
                args.push(Arg {
                    ident: String::from_utf8(token.literal).unwrap(), 
                    kind: Type {name: String::from_utf8(type_token.literal).unwrap()}});
            }
            else if token.ttype == TToken::CPAREN {break;}
        }
        let token = expect_token(lexer, vec![TToken::Identifier]);
        let return_type = Type { name: String::from_utf8(token.literal).unwrap() };
        expect_token(lexer, vec![TToken::OCURLY]);
        expect_token(lexer, vec![TToken::CCURLY]);
        return Self { ident , args, return_type, block: Vec::new() } ;
    }
    pub fn get_node(lexer: &mut Lexer) -> Node {
        Node::Func { var: Self::new(lexer) }
    }

}

#[derive(Debug)]
enum Node {
    Func {var: Func},
    VariableDelclear { var: VariableDelclear }
}


pub fn expect_token(lexer: &mut Lexer, types:Vec<TToken>) -> Token {
    let token = lexer.next_token();
    
    if token.ttype == TToken::EOF {
        println!("expect one of {:?} found EOF {}:{}:{}",types,token.file_path,token.line,token.col);
        exit(1);
    }
    if types.contains(&token.ttype) {
        return token;
    }else {
        println!("expect one of {:?} found {:?} {}:{}:{}",types,token.ttype,token.file_path,token.line,token.col);
        exit(1);
    }
}


fn main() {
    let mut lexer = Lexer::from_str("@hello = \"facts\";\nfun main(a b,c d) u32 {}");
    let program = Program::new(&mut lexer);
    println!("{:#?}",program);
}

#[cfg(test)]
mod parser_tests {
    use crate::VariableDelclear;
    use crate::Type;
    use crate::parser::Lexer;

    #[test]
    fn dynamic_variable_declearation() {
        let mut lexer = Lexer::from_str("hello u32;\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "".to_string(),
        });
        let mut lexer = Lexer::from_str("hello = \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "undifiend".to_string(),
            },
            init_value: "facts".to_string(),
        });
        let mut lexer = Lexer::from_str("hello u32 = \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: false,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }

    #[test]
    fn const_variable_declearation() {
        let mut lexer = Lexer::from_str("hello : \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "undifiend".to_string(),
            },
            init_value: "facts".to_string(),
        });
        let mut lexer = Lexer::from_str("hello u32 : \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: false,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }

    #[test]
    fn static_variable_declearation() {
        let mut lexer = Lexer::from_str("hello u32 :: \"facts\";\n");
        assert_eq!(VariableDelclear::new(&mut lexer),VariableDelclear{
            is_const: true,
            is_static: true,
            ident: "hello".to_string(),
            kind: Type {
                name: "u32".to_string(),
            },
            init_value: "facts".to_string(),
        });
    }
}
