use crate::tokenized::Operators;

impl Operators {
    pub fn bin_precedence(&self) -> Option<u8> {
        match self {
            Operators::Pipe => Some(1),                                       
            Operators::Ampersand => Some(2),                                  
            Operators::Equal | Operators::DoubleEqual | Operators::NotEqual |
            Operators::LessThan | Operators::GreaterThan | 
            Operators::LessEqual | Operators::GreaterEqual => Some(3),   
            Operators::RangeDots => Some(4),                                         
            // Operators::Semicolon => Some(5),  
            Operators::Plus | Operators::Minus => Some(6),
            Operators::Star | Operators::Slash => Some(7),
            Operators::Caret => Some(9),
            Operators::LBracket => Some(10),                               
            _ => None,
        }
    }

    pub fn unary_precedence(&self) -> Option<u8> {
        match self {
            Operators::Minus | Operators::Plus | Operators::Exclamation => Some(8),
            _ => None,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        matches!(self, Operators::Caret | Operators::Equal)
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Integer(i64),
    Number(f64),
    Complex(f64, f64),
    Variable(String),
    Constant(String),
    Range{start: Box<Node>, end: Box<Node>, step: Option<Box<Node>>},
    Matrix{rows: Vec<Vec<Node>>},

    UnaryOp{op: Operators, child: Box<Node>},
    BinOp{op: Operators, left: Box<Node>, right: Box<Node>},

    FunctionCall{name: String, args: Box<Vec<Node>>},
    FunctionDef{name: String, params: Vec<String>, body: Box<Node>},

    Index{expr: Box<Node>, indices: Vec<Node>},
    EmptySlice,

    Summation{var: String, start: Box<Node>, end: Box<Node>, body: Box<Node>},
    Product{var: String, start: Box<Node>, end: Box<Node>, body: Box<Node>},

    DerivativeExpr{var: String, order: Box<Node>, body: Box<Node>},
    
    DefIntegral{var: String, start: Box<Node>, end: Box<Node>, body: Box<Node>},
    IndefIntegral{var: String, body: Box<Node>},
    Limit{var: String, target: Box<Node>, body: Box<Node>}
}