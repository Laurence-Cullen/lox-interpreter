use crate::tokens::Token;

pub trait Expr {
    fn print(&self) -> String;
    fn eval(&self) -> Box<dyn Expr>;
}

pub struct Binary {
    lhs: Box<dyn Expr>,
    op: Token,
    rhs: Box<dyn Expr>,
}

impl Binary {
    pub(crate) fn new(lhs: Box<dyn Expr>, op: Token, rhs: Box<dyn Expr>) -> Box<Self> {
        Box::new(Binary { lhs, op, rhs })
    }
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

    fn eval(&self) -> Box<dyn Expr> {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        // If lhs and rhs are both NumberLiterals add
        match (lhs, rhs) {
            (Expressions::Number(ref l_val), Expressions::Number(ref r_val)) => {
                match self.op {
                    Token::Plus => {
                        NumberLiteral::new(l_val.value + r_val.value)
                    },
                    Token::Minus => {
                        NumberLiteral::new(l_val.value - r_val.value)
                    },
                    Token::Star => {
                        NumberLiteral::new(l_val.value * r_val.value)
                    },
                    Token::Slash => {
                        NumberLiteral::new(l_val.value / r_val.value)
                    },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
            }
        }
    }


// pub struct Call {
//     callee: Box<dyn Expr>,
//     paren: Token,
//     args: Vec<Box<dyn Expr>>,
// }
//
// impl Call {
//     fn new(callee: Box<dyn Expr>, paren: Token, args: Vec<Box<dyn Expr>>) -> Box<Self> {
//         Box::new(Call {
//             callee,
//             args,
//             paren,
//         })
//     }
// }
//
// impl Expr for Call {
//     fn print(&self) -> String {
//         let mut args_str = "".to_string();
//
//         for arg in &self.args {
//             args_str += format!("{}, ", arg.print()).as_ref();
//         }
//         format!("{}({})", self.callee.print(), args_str)
//     }
//     fn eval(&self) -> Box<dyn Expr> {
//
//     }
// }

// struct Get {
//     object: Box<dyn Expr>,
//     name: Token,
// }
// impl Expr for Get {
//     fn print(&self) -> String {
//         match self.name {
//             Token::Identifier(ref name) => format!("{}.{}", self.object.print(), name),
//             _ => unreachable!(),
//         }
//     }
// }

pub struct Grouping {
    expr: Box<dyn Expr>,
}

impl Grouping {
    fn new(expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(Grouping { expr })
    }
}

impl Expr for Grouping {
    fn print(&self) -> String {
        format!("({})", self.expr.print())
    }

    fn eval(&self) -> Box<dyn Expr> {
        self.expr.eval()
    }
}

pub struct StringLiteral {
    value: String,
}
impl StringLiteral {
    fn new(value: String) -> Box<Self> {
        Box::new(StringLiteral { value })
    }
}

impl Expr for StringLiteral {
    fn print(&self) -> String {
        self.value.clone()
    }
    fn eval(&self) -> Box<dyn Expr> {
        Self::new(self.value.clone())
    }
}

pub struct NumberLiteral {
    value: f64,
}
impl NumberLiteral {
    pub(crate) fn new(value: f64) -> Box<Self> {
        Box::new(NumberLiteral { value })
    }
}

impl Expr for NumberLiteral {
    fn print(&self) -> String {
        self.value.to_string()
    }
    fn eval(&self) -> Box<dyn Expr> {
        NumberLiteral::new(self.value)
    }
}

pub struct BooleanLiteral {
    value: bool,
}
impl BooleanLiteral {
    pub(crate) fn new(value: bool) -> Box<Self> {
        Box::new(BooleanLiteral { value })
    }
}
impl Expr for BooleanLiteral {
    fn print(&self) -> String {
        self.value.to_string()
    }
    fn eval(&self) -> Box<dyn Expr> {
        BooleanLiteral::new(self.value)
    }
}

pub struct Logical {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Logical {
    fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Logical {
            left,
            operator,
            right,
        })
    }
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

        format!("{} {:?} {}", self.left.print(), op_str, self.right.print())
    }
    fn eval(&self) -> Box<dyn Expr> {
        let left = self.left.eval();
        let right = self.right.eval();

        match (left, right) {
            (Expressions::Number(ref l_val), Expressions::Number(ref r_val)) => {
                match self.operator {
                    Token::BangEqual => {
                        BooleanLiteral::new(l_val.value != r_val.value)
                    }
                    Token::EqualEqual => {
                        BooleanLiteral::new(l_val.value == r_val.value)
                    }
                    Token::Greater => {
                        BooleanLiteral::new(l_val.value > r_val.value)
                    }
                    Token::GreaterEqual => {
                        BooleanLiteral::new(l_val.value >= r_val.value)
                    }
                    Token::Less => {
                        BooleanLiteral::new(l_val.value <= r_val.value)
                    }
                    Token::LessEqual => {
                        BooleanLiteral::new(l_val.value <= r_val.value)
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
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

impl Unary {
    fn new(operator: Token, right: Box<dyn Expr>) -> Box<Self> {
        Box::new(Unary { operator, right })
    }
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
    fn eval(&self) -> Box<dyn Expr> {
        let right = self.right.eval();

        match right {
            Expressions::Number(ref val) => {
                match self.operator {
                    Token::Minus => {
                        NumberLiteral::new(-val.value)
                    },
                    _ => unreachable!(),
                }
            },
            Expressions::Boolean(ref val) => {
                match self.operator {
                    Token::Bang => {
                        BooleanLiteral::new(!val.value)
                    },
                    _ => unreachable!(),
                }
            }
        }

    }
}

struct Variable {
    name: Token,
}

// impl Variable {
//     fn new(name: Token) -> Box<Self> {
//         Box::new(Variable { name })
//     }
// }
//
// impl Expr for Variable {
//     fn print(&self) -> String {
//         match &self.name {
//             Token::Identifier(thing) => thing.to_owned(),
//             _ => unreachable!(),
//         }
//     }
// }

enum Expressions {
    // Call(Box<Call>),
    Grouping(Box<Grouping>),
    StringLiteral(Box<StringLiteral>),
    Number(Box<NumberLiteral>),
    Boolean(Box<BooleanLiteral>),
    Logical(Box<Logical>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let tree = Binary::new(
            Unary::new(
                Token::Minus,
                Grouping::new(Binary::new(
                    NumberLiteral::new(1.0),
                    Token::Slash,
                    NumberLiteral::new(2.0),
                )),
            ),
            Token::Minus,
            NumberLiteral::new(2.0),
        );

        println!("{}", tree.print());
    }
}


