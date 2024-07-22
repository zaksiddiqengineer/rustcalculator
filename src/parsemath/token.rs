/// this contains enum for list of tokens and handles operator precendence rules

// List of valid tokens that can be constructured from arithmatic expression by Tokenizer
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

// order of operators as per operator precendence rules
#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl  Token{
    pub fn get_oper_prec(&self) -> OperPrec{
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,
            _ => DefaultZero,
        }
    }
}