use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::{Expr, Operator};
use crate::operator::Combinator;
use crate::subclause::Subclause;

pub trait Matchable: Display {
    fn matches(&self, map: &HashMap<String,String>) -> bool;
}

pub struct Query {
    pub bracketed: bool,
    pub left: Box<dyn Matchable>,
    pub right: Option<Subclause>,
}

impl Matchable for Query {
    fn matches(&self, map: &HashMap<String, String>) -> bool {
        let left = self.left.matches(map);
        match &self.right {
            Some(subclause) => subclause.evaluate(left, map),
            _ => left
        }
    }
}

impl Query {
    pub fn and(self, clause: impl Matchable + 'static) -> Self {
        Query {
            right: Some(Subclause::new(Combinator::And,
                                       clause)),
            ..self
        }
    }
    pub fn or(self, clause: impl Matchable + 'static) -> Self {
        Query {
            right: Some(Subclause::new(Combinator::Or,
                                       clause)),
            ..self
        }
    }
    pub fn expression(field_name: String, operator: Operator, value: String) -> Self {
        Query { bracketed: false, left: Box::new(Expr::new(field_name, operator, value)), right: None }
    }
    pub fn bracketed_expression(field_name: String, operator: Operator, value: String) -> Self {
        Query { bracketed: true, left: Box::new(Expr::new(field_name, operator, value)), right: None }
    }
    pub fn compound(left: impl Matchable + 'static) -> Self {
        Query { bracketed: false, left: Box::new(left), right: None }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.bracketed {
            true =>
                match &self.right {
                    Some(sub) => write!(f, "({} {})", self.left, sub),
                    _ => write!(f, "({})", self.left)
                },
            false =>
                match &self.right {
                    Some(sub) => write!(f, "{} {}", self.left, sub),
                    _ => write!(f, "{}", self.left)
                }
        }
    }
}
