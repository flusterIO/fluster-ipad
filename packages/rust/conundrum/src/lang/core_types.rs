#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add, // +
    Sub, // -
    Mul, // *
    Mod, // %
    Div, // /
    Pow, // ^ or **
    Eq,  // ==
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expr,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    MarkdownBridge(String), // The "MDX" part: raw markdown between code
}
