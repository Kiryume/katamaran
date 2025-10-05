use derivative::Derivative;

pub trait Pos {
    fn pos(&self) -> (usize, usize);
}

#[derive(Debug)]
pub enum Statement {
    Be(BeStatement),
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

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ident {
    pub name: String,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

impl Pos for Statement {
    fn pos(&self) -> (usize, usize) {
        match self {
            Statement::Be(be_stmt) => be_stmt.pos(),
        }
    }
}

impl Pos for BeStatement {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Pos for Expression {
    fn pos(&self) -> (usize, usize) {
        match self {
            Expression::Ident(ident) => ident.pos(),
        }
    }
}

impl Pos for Ident {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}
