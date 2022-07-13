use std::fmt::Display;

#[derive(Debug)]
pub enum LambdaExpr {
    Variable(String),
    Abstraction(String, Box<LambdaExpr>),
    Application(Box<LambdaExpr>, Box<LambdaExpr>),
}

impl Display for LambdaExpr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambdaExpr::Variable(x) => write!(fmt, "{}", x),
            LambdaExpr::Abstraction(x, b) => write!(fmt, "\\{}.({})", x, b),
            LambdaExpr::Application(f, a) => write!(fmt, "({}) ({})", f, a),
        }
    }
}
