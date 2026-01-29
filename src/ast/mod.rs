use crate::types::SrcSpan;

pub struct Stmt {
    pub kind: StmtKind,
    pub span: SrcSpan,
}

pub enum StmtKind {
    Let(Ident),
    Fn(Fn),
    Expr(Expr),
}

pub struct Let {
    pub ident: Ident,
    pub expr: Expr,
    pub span: SrcSpan,
}

pub struct Fn {
    pub ident: Ident,
    pub args: Vec<Ident>,
    pub body: Expr,
    pub span: SrcSpan,
}

pub struct Expr {
    pub kind: ExprKind,
    pub span: SrcSpan,
}

pub enum ExprKind {
    Ident(Ident),
    Fn(AnonFn),
    Block(Block),
    Lit(Lit),
    Match(Match),
    UnOp(UnOp),
    BinOp(BinOp),
}

pub struct Ident {
    pub name: String,
    pub span: SrcSpan,
}

pub struct AnonFn {
    pub args: Vec<Ident>,
    pub body: Block,
    pub span: SrcSpan,
}

pub struct Block {
    pub exprs: Vec<Expr>,
    pub span: SrcSpan,
}

pub struct Lit {
    pub lit: LitKind,
    pub span: SrcSpan,
}

pub enum LitKind {
    Bool(bool),
    Str(String),
    Int(i64),
    Float(f64),
}

pub struct Match {
    pub against: Box<Expr>,
    pub arms: Vec<Arm>,
    pub span: SrcSpan,
}

pub struct Arm {
    pub pat: MatchPat,
    pub expr: Expr,
    pub span: SrcSpan,
}

pub enum MatchPat {
    Lit(Lit),
    Under(SrcSpan),
}

pub struct UnOp {
    pub op: UnOpKind,
    pub expr: Box<Expr>,
    pub span: SrcSpan,
}

pub enum UnOpKind {
    Not,
}

pub struct BinOp {
    pub left: Box<Expr>,
    pub op: BinOpKind,
    pub right: Box<Expr>,
    pub span: SrcSpan,
}

pub enum BinOpKind {
    Pipe,
    Dot,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    EqTo,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,

    And,
    Or,
    Not,
}
