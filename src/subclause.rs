use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::operator::Combinator;
use crate::query::Matchable;

pub struct Subclause {
    combinator: Combinator,
    clause: Box<dyn Matchable>,
}

impl Subclause {
    pub fn new(combinator: Combinator, clause: impl Matchable + 'static) -> Self {
        Subclause { combinator, clause: Box::new(clause) }
    }
    pub fn evaluate(&self, left: bool, map: &HashMap<String, String>) -> bool {
        match &self.combinator {
            Combinator::And => left && self.clause.matches(map),
            Combinator::Or =>  left || self.clause.matches(map)
        }
    }
}

impl Display for Subclause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.combinator, self.clause)
    }
}
