use std::fmt::{Display, Formatter};

pub enum Operator {
    Equals,
    Like,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "=")
    }
}

pub enum Combinator {
    And,
    Or,
}

impl Display for Combinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Combinator::And => write!(f, "AND"),
            Combinator::Or => write!(f, "OR"),
        }
    }
}
