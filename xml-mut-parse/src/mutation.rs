use crate::prelude::*;
use nom::{character::complete::multispace1, combinator::opt, IResult};

pub fn mutation(s: &str) -> IResult<&str, Mutation> {
    let (s, get) = get_statement(s)?;
    let (s, _) = multispace1(s)?;
    let (s, where_clause) = where_clause(s)?;
    let (s, _) = multispace1(s)?;
    let (s, set) = opt(set_statement)(s)?;
    let (s, _) = multispace1(s)?;
    let (s, delete) = opt(delete_statement)(s)?;

    Ok((
        s,
        Mutation {
            get,
            where_clause,
            set,
            delete,
        },
    ))
}
