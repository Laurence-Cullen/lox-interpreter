use crate::tokens::Token;

pub trait Expr {
    fn print(&self) -> String;
}

struct Binary {
    lhs: Box<dyn Expr>,
    op: Token,
    rhs: Box<dyn Expr>,
}

impl Expr for Binary {
    fn print(&self) -> String {
        let op_str = match &self.op {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            _ => unreachable!(),
        };

        format!("{} {} {}", self.lhs.print(), op_str, self.rhs.print())
    }
}

struct Call {
    callee: Box<dyn Expr>,
    paren: Token,
    args: Vec<Box<dyn Expr>>,
}
impl Expr for Call {
    fn print(&self) -> String {
        let mut args_str = "".to_string();

        for arg in &self.args {
            args_str += format!("{}, ", arg.print()).as_ref();
        }
        format!("{}({})", self.callee.print(), args_str)
    }
}

struct Get {
    object: Box<dyn Expr>,
    name: Token,
}
impl Expr for Get {
    fn print(&self) -> String {
        match self.name {
            Token::Identifier(ref name) => format!("{}.{}", self.object.print(), name),
            _ => unreachable!(),
        }
    }
}

struct Grouping {
    expr: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn print(&self) -> String {
        format!("({})", self.expr.print())
    }
}

struct StringLiteral {
    value: String,
}

impl Expr for StringLiteral {
    fn print(&self) -> String {
        self.value.clone()
    }
}

struct NumberLiteral {
    value: f64,
}

impl Expr for NumberLiteral {
    fn print(&self) -> String {
        self.value.to_string()
    }
}

struct Logical {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Logical {
    fn print(&self) -> String {

        let op_str = match &self.operator {
            Token::EqualEqual => "==",
            Token::GreaterEqual => ">=",
            Token::LessEqual => "<=",
            Token::Greater => ">",
            Token::Less => "<",
            _ => unreachable!(),
        };

        format!(
            "{} {:?} {}",
            self.left.print(),
            op_str,
            self.right.print()
        )
    }
}

// struct Super {
//     keyword: Token,
//     method: Token,
// }
//
// impl Expr for Super {
//     fn print() {}
// }
//
// struct This {
//     keyword: Token,
// }
//
// impl Expr for This {
//     fn print() {}
// }

struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn print(&self) -> String {
        let op_str = match &self.operator {
            Token::Bang => "!",
            Token::Minus => "-",
            _ => unreachable!(),

        };
        format!("{} {}", op_str, self.right.print())
    }
}

struct Variable {
    name: Token,
}

impl Expr for Variable {
    fn print(&self) -> String {
        match &self.name {
            Token::Identifier(thing) => { thing.to_owned() },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let tree = Binary{
            lhs: Box::new((NumberLiteral{value: 0.0})),
            op: Token::Plus,
            rhs: Box::new((Unary{ operator: Token::Minus, right: Box::new((NumberLiteral{value: 0.0})) })),

        };

        println!("{}", tree.print());
    }
}
