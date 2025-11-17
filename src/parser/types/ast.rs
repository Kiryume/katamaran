use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(Debug)]
pub enum StmtKind {
    Be(BeStmt),
    Return(ReturnStmt),
    Expression(ExpressionStmt),
}

#[derive(Debug)]
pub struct BeStmt {
    pub ident: Ident,
    pub value: Expression,
    pub is_mut: bool,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expr: Expression,
    pub pos: (usize, usize),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(Debug)]
pub enum ExpressionKind {
    Ident(Ident),
    Literal(LiteralExpr),
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct LiteralExpr {
    pub value: Literal,
}
