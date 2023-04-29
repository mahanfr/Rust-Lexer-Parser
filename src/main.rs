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

#[derive(Debug)]
struct Type {
    name: String,
}

#[derive(Debug)]
struct VariableDelclear {
    is_const: bool,
    is_static: bool,
    ident: String,
    kind: Type,
    // TODO: Change this shit
    init_value: String,
}
impl VariableDelclear {
    fn new(lexer: &mut Lexer) -> Self {
        let mut token = expect_token(lexer, vec![TToken::Identifier]);
        let ident = token.get_literal_string();
        token = expect_token(lexer, vec![TToken::Identifier,TToken::COLON,TToken::EQ]);
        let kind: Type;
        let is_const: bool;
        let is_static: bool;
        if token.ttype == TToken::Identifier {
            kind = Type { name: token.get_literal_string() };
            token = expect_token(lexer, vec![TToken::COLON,TToken::EQ,TToken::SEMICOLON]);
        }else{
            kind = Type { name: "undifiend".to_string() };
            token = expect_token(lexer, vec![TToken::COLON,TToken::EQ,TToken::SEMICOLON]);
        }
        if token.ttype == TToken::SEMICOLON {
            return Self{is_const: false, is_static:false, ident, kind, init_value: String::new()};
        }

        if token.ttype == TToken::COLON {
            is_const = true;
            token = expect_token(lexer, vec![TToken::COLON,TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
            if token.ttype == TToken::COLON {
                is_static = true;
                token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
            }else{
                is_static = false;
            }
        } else {
            is_const = false;
            is_static = false;
            token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
        }
        let init_value = token.get_literal_string();
        expect_token(lexer, vec![TToken::SEMICOLON]);
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
    let mut lexer = Lexer::from_str("@hello u32 = \"facts\";\nfun main(a b,c d) u32 {}");
    let program = Program::new(&mut lexer);
    println!("{:#?}",program);
}
