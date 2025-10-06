use derivative::Derivative;

pub trait Pos {
    fn pos(&self) -> (usize, usize);
}

#[derive(Debug)]
pub enum Statement {
    Be(BeStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BeStatement {
    pub ident: Ident,
    pub value: Expression,
    pub is_mut: bool,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ReturnStatement {
    pub value: Expression,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(LiteralExpr),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ident {
    pub name: String,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
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
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

impl Pos for Statement {
    fn pos(&self) -> (usize, usize) {
        match self {
            Statement::Be(be_stmt) => be_stmt.pos(),
            Statement::Return(ret_stmt) => ret_stmt.pos(),
            Statement::Expression(expr) => expr.pos(),
        }
    }
}

impl Pos for BeStatement {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Pos for ReturnStatement {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Pos for ExpressionStatement {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Pos for Expression {
    fn pos(&self) -> (usize, usize) {
        match self {
            Expression::Ident(ident) => ident.pos(),
            Expression::Literal(lit) => lit.pos(),
        }
    }
}

impl Pos for Ident {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Pos for LiteralExpr {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}
