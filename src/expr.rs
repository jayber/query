use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::operator::Operator;
use crate::query::Matchable;

pub struct Expr {
    pub field_name: String,
    pub operator: Operator,
    pub value: String,
}

impl Matchable for Expr {
    fn matches(&self, map: &HashMap<String, String>) -> bool {
        match map.get(self.field_name.as_str()) {
            Some(val) => val == self.value.as_str(),
            _ => false
        }
    }
}

impl Expr {
    pub fn new(field_name: String, operator: Operator, value: String) -> Self {
        Expr { field_name, operator, value }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.field_name, self.operator, self.value)
    }
}
