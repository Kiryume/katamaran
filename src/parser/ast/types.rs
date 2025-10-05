use derivative::Derivative;

#[derive(Debug)]
pub enum Statement {
    Let(BeStatement),
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
