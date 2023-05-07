extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct NemetParser;

struct ProgramFile {
    file_path: String,
    functions: Vec<Func>,
    variables: Vec<StaticVariableDeclear>,
}
impl ProgramFile{
    pub fn new(file_path: String) -> Self {
        Self {file_path, functions: Vec::new(), variables: Vec::new()}
    }
}

struct Func {
    ident: String,
    // TODO: args
    return_type: Type,
    block : Vec<Stmt>,
}
impl Func {
    pub fn new(ident: String, return_type: Type) -> Self {
        Self { ident, return_type, block: Vec::new() }
    }

}

struct StaticVariableDeclear {
    ident: String,
    value_type: Type,
    value: Expr
}
impl StaticVariableDeclear{
    pub fn new(ident: String, value_type: Type, value: Expr) -> Self {
        Self {ident, value_type, value}
    }
    pub fn from_pair(pairs: Pairs<Rule>) -> Self {
        let mut pairs = pairs.into_iter();
        let ident = pairs.next().unwrap().to_string();
        let value_type = Type::get(pairs.next().unwrap().to_string());
        //TODO: Parse Expr
        let value = Expr::Int(0);
        
        Self { ident, value_type, value }
    }
}

enum Expr {
    Ident(String),
    Int(u32),
    Float(f32),
    //IndexingExpr(),
    Unary(Op, Box<Expr>),
    Binary{
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>
    }
}

enum Op {
    Add,
    Sub,
    Multi,
    Divide,
    Mod,
    And,
    Or,
    Xor,
    Lsh,
    Rsh,
}
impl Op {
    pub fn get(lit: String) -> Self{
        match lit.as_str() {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Multi,
            "/" => Self::Divide,
            "%" => Self::Mod,
            "&" => Self::And,
            "|" => Self::Or,
            "^" => Self::Xor,
            "<<" => Self::Lsh,
            ">>" => Self::Rsh,
            _ => unreachable!("Undifiend Operand"), 
        }
    }

}

enum Type{
    Ident(String),
    I,
    F,
    Bool,
    Void,
}
impl Type {
    pub fn get(name: String) -> Self{
        match name.as_str() {
            "@i" => Self::I,
            "@f" => Self::F,
            "@bool" => Self::Bool,
            _ => Self::Ident(name.replace("@", ""))
        }
    }
}

enum Stmt{
    Assign(Assign),
    LocalVariable(LocalVariable),
    DubugPrint(Expr),
}

struct LocalVariable {
    is_const: bool,
    ident: String,
    value_type: Option<Type>,
    value: Option<Expr>
}
impl LocalVariable{
    pub fn new(is_const: bool, ident: String, value_type: Option<Type>, value: Option<Expr>) -> Self {
        Self {is_const, ident, value_type, value}
    }
}

struct Assign {
    lhs: String,
    op: AssignOp,
    rhs: Expr,
}
impl Assign {
    pub fn new(lhs: String, op: AssignOp, rhs: Expr) -> Self {
        Self { lhs, op, rhs }
    } 

}

enum AssignOp {
    Eq,
    AddEq,
    SubEq,
    MultiEq,
    DivideEq,
    ModEq,
    AndEq,
    OrEq,
    XorEq,
}
impl AssignOp {
    pub fn get(lit: String) -> Self{
        match lit.as_str() {
            "=" => Self::Eq,
            "+=" => Self::AddEq,
            "-=" => Self::SubEq,
            "*=" => Self::MultiEq,
            "/=" => Self::DivideEq,
            "%=" => Self::ModEq,
            "&=" => Self::AndEq,
            "|=" => Self::OrEq,
            "^=" => Self::XorEq,
            _ => unreachable!("Undifiend Operand"), 
        }
    }
}

fn main() {
    let pairs = NemetParser::parse(Rule::program_file, 
            "static name @u32 :: 110 + a - 2;\nfunc m2() {} \n"
        )
        .unwrap_or_else(|e| panic!("{}", e));
    let mut program = ProgramFile::new("INTERNAL".to_string());
    for pair in pairs {
        if pair.as_rule() == Rule::static_variable_declear {
            program.variables.push(StaticVariableDeclear::from_pair(pair.into_inner()));
        }else if pair.as_rule() == Rule::function_defin {
            todo!()
        }else {
            unreachable!("Program File Can Only Define Static Variables And Functions");
        }
    }

}

