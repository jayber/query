use std::collections::HashMap;
use expr::Expr;
use operator::Operator;
use query::Query;
use crate::Operator::Equals;
use crate::query::Matchable;

mod expr;
mod operator;
mod query;
mod subclause;

fn main() {
    test_as_strings();

    test_as_matching_records();
}

fn test_as_matching_records() {
    let mut record = HashMap::new();
    record.insert(String::from("name"), "bart".to_string());
    record.insert(String::from("parent"), "homer".to_string());

    assert!(Expr { field_name: "name".to_string(), operator: Equals, value: "bart".to_string() }.matches(&record));
    assert!(!Expr { field_name: "name".to_string(), operator: Equals, value: "lisa".to_string() }.matches(&record));

    assert!(Query::expression("name".to_string(), Equals, "bart".to_string()).matches(&record));
    assert!(!Query::expression("name".to_string(), Equals, "lisa".to_string()).matches(&record));

    assert!(Query::expression("name".to_string(), Equals, "bart".to_string())
        .and(Expr { field_name: "name".to_string(), operator: Equals, value: "bart".to_string() }).matches(&record));
    assert!(!Query::expression("name".to_string(), Equals, "bart".to_string())
        .and(Expr { field_name: "name".to_string(), operator: Equals, value: "lisa".to_string() }).matches(&record));

    assert!(Query::expression("name".to_string(), Equals, "bart".to_string())
        .and(Query::expression("name".to_string(), Equals, "lisa".to_string())
            .or(Expr { field_name: "name".to_string(), operator: Equals, value: "bart".to_string() })).matches(&record));

    assert!(Query::expression("name".to_string(), Equals, "bart".to_string())
        .or(Query::expression("name".to_string(), Equals, "lisa".to_string())
            .and(Expr { field_name: "parent".to_string(), operator: Equals, value: "marge".to_string() })).matches(&record));

    assert!(!Query::compound(
        Query::bracketed_expression("name".to_string(), Equals, "bart".to_string())
            .or(Query::expression("name".to_string(), Equals, "lisa".to_string())))
        .and(Expr { field_name: "parent".to_string(), operator: Equals, value: "marge".to_string() }).matches(&record), "bracketing");

    // test fails, behaves like every query is bracketed
    assert!(Query::compound(
        Query::expression("name".to_string(), Equals, "bart".to_string())
            .or(Query::expression("name".to_string(), Equals, "lisa".to_string())))
                .and(Expr { field_name: "parent".to_string(), operator: Equals, value: "marge".to_string() }).matches(&record), "non bracketing");
}

fn test_as_strings() {
    assert_eq!("app = slack",
               Query::expression("app".to_string(), Equals, "slack".to_string()).to_string());
    assert_eq!("app = slack AND (field1 = general OR field2 = chat)",
               Query::expression("app".to_string(), Equals, "slack".to_string())
                   .and(Query::bracketed_expression("field1".to_string(), Equals, "general".to_string())
                       .or(Expr::new("field2".to_string(), Equals, "chat".to_string()))
                   ).to_string());
    assert_eq!("app = slack AND (field1 = general OR field2 = chat) AND (field3 = super OR field4 = pulvy)",
               Query::expression("app".to_string(), Equals, "slack".to_string())
                   .and(Query::compound(
                       Query::bracketed_expression("field1".to_string(), Equals, "general".to_string())
                           .or(Query::expression("field2".to_string(), Equals, "chat".to_string())))
                       .and(Query::bracketed_expression("field3".to_string(), Equals, "super".to_string())
                           .or(Expr { field_name: "field4".to_string(), operator: Equals, value: "pulvy".to_string() }))).to_string());
    assert_eq!("app = slack AND (field1 = general OR field2 = chat OR field5 = plunk) AND (field3 = super OR field4 = pulvy) AND fieldx = z",
               Query::expression("app".to_string(), Equals, "slack".to_string())
                   .and(Query::compound(
                       Query::bracketed_expression("field1".to_string(), Equals, "general".to_string())
                           .or(Query::expression("field2".to_string(), Equals, "chat".to_string())
                               .or(Expr { field_name: "field5".to_string(), operator: Equals, value: "plunk".to_string() })))
                       .and(Query::compound(
                           Query::bracketed_expression("field3".to_string(), Equals, "super".to_string())
                               .or(Expr { field_name: "field4".to_string(), operator: Equals, value: "pulvy".to_string() }))
                           .and(Expr { field_name: "fieldx".to_string(), operator: Equals, value: "z".to_string() }))).to_string());
}



