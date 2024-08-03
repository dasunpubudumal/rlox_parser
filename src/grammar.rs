use rlox_lib::token::{Token, TokenType};

/// Abstract concept Expression that is either a:
/// - literal
/// - unary
/// - binary
/// - grouping
pub trait Expression {}

// dyn Expression is a dynamically dispatched implementation of a trait.
// The compiler does not know what type it is. It is the runtime that figures it out.
// This substitiute for an abstract class, as I've used here is a trait. Since the space required for the 
// trait object cannot be identified at compile time, we need to dynamically dispatch it.

/// binary         → expression operator expression ;
pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>
}

impl Binary {
    pub fn new(left: Box<dyn Expression>, operator: String, right: Box<dyn Expression>) -> Self {
        Self {
            left, operator, right
        }
    }
}

/// grouping       → "(" expression ")" ;
pub struct Grouping {
    pub expression: Box<dyn Expression>
}

impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression
        }
    }
}

/// unary          → ( "-" | "!" ) expression ;
pub struct Unary {
    pub operator: Token<String>,
    pub expression: Box<dyn Expression>
}

impl Expression for Binary {}
impl Expression for Grouping {}
impl Expression for Unary {}