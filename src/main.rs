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
                body.push(Func::new(lexer));
            }else if token.ttype == TToken::ATSIGN {
                todo!("Not Implemented");
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
    pub fn new(lexer: &mut Lexer) -> Node {
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
        return Node::Func { var: Func { ident , args, return_type, block: Vec::new() } };
    }

}

#[derive(Debug)]
enum Node {
    Program {var: Program},
    Func {var: Func},
    Type {var: Type},
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
    let mut lexer = Lexer::from_str("fun main(a b,c d) u32 {}");
    let program = Program::new(&mut lexer);
    println!("{:#?}",program);
}
