use crate::prelude::*;
use nom::{character::complete::multispace1, combinator::opt, sequence::preceded, IResult};
use xml_mut_data::Mutation;

pub fn mutation(s: &str) -> IResult<&str, Mutation> {
    let (s, get_clause) = get_clause(s)?;
    let (s, where_clause) = opt(preceded(multispace1, where_clause))(s)?;
    let mem = s;
    let (s, set_clause) = opt(preceded(multispace1, set_clause))(s)?;
    let (s, delete_clause) = opt(preceded(multispace1, delete_clause))(s)?;

    if set_clause.is_none() && delete_clause.is_none() {
        return Err(nom::Err::Error(nom::error::Error {
            code: nom::error::ErrorKind::Permutation,
            input: mem,
        }));
    }

    Ok((
        s,
        Mutation {
            get_clause,
            where_clause,
            set_clause,
            delete_clause,
        },
    ))
}
