use crate::tokens::Token;

pub trait Expr {
    fn print(&self) -> String;
}

struct Binary {
    lhs: dyn Expr,
    op: Token,
    rhs: dyn Expr,
}
impl Expr for Binary {
    fn print(&self) -> String {
        let op_str =  match &self.op {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Equal => "==",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            _ => unreachable!(),
        };

        format!("{} {} {}", self.lhs, op_str, self.rhs)
    }
}

struct Call {
    callee: dyn Expr,
    paren: Token,
    args: Vec<dyn Expr>,
}
impl Expr for Call {
    fn print(&self) -> String {
        let mut args_str = "".to_string();

        for arg in &self.args {
            args_str += format!("{}, ", arg).as_ref();
        }
        format!("{}({})", self.callee.print(), args_str)
    }
}

struct Get {
    object: dyn Expr,
    name: Token,
}
impl Expr for Get {
    fn print(&self) -> String {
        format!("{}.{}",
}

struct Grouping {
    expr: dyn Expr,
}

impl Expr for Grouping {
    fn print() {}
}

struct StringLiteral {
    value: String,
}

impl Expr for StringLiteral {
    fn print() {}
}

struct NumberLiteral {
    value: f64,
}

impl Expr for NumberLiteral {
    fn print() {}
}

struct Logical {
    left: dyn Expr,
    operator: Token,
    right: dyn Expr,
}

impl Expr for Logical {
    fn print() {}
}

struct Set {
    object: dyn Expr,
    name: Token,
    value: dyn Expr,
}

impl Expr for Set {
    fn print() {}
}

struct Super {
    keyword: Token,
    method: Token,
}

impl Expr for Super {
    fn print() {}
}

struct This {
    keyword: Token,
}

impl Expr for This {
    fn print() {}
}

struct Unary {
    operator: Token,
    right: dyn Expr,
}

impl Expr for Unary {
    fn print() {}
}

struct Variable {
    name: Token,
}

impl Expr for Variable {
    fn print() {}
}
