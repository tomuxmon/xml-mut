use crate::prelude::*;
use nom::{character::complete::multispace1, combinator::opt, sequence::preceded, IResult};
use xml_mut_data::Mutation;

pub fn mutation(s: &str) -> IResult<&str, Mutation> {
    let (s, get) = get_statement(s)?;
    let (s, where_clause) = opt(preceded(multispace1, where_clause))(s)?;
    let (s, set) = opt(preceded(multispace1, set_statement))(s)?;
    let (s, delete) = opt(preceded(multispace1, delete_statement))(s)?;

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
