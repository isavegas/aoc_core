use std::str::FromStr;
use crate::ErrorWrapper;

// I would like to use `P where P: Pattern`, but it interferes with
// generic typing for function.
pub fn parse<N>(input: &str, delimiter: &str) -> Result<Vec<N>, ErrorWrapper>
where
    N: FromStr,
    ErrorWrapper: From<<N as FromStr>::Err>,
{
    input
        .trim()
        .split(delimiter)
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .try_fold(vec![], |mut v, r| {
            v.push(r?);
            Ok(v)
        })
}

pub fn parse_with<N>(input: &str, delimiter: &str, func: fn(&str) -> Result<N, ErrorWrapper>) -> Result<Vec<N>, ErrorWrapper> {
    input
        .trim()
        .split(delimiter)
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(func)
        .try_fold(vec![], |mut v, r| {
            v.push(r?);
            Ok(v)
        })
}

pub fn parse_lines<N>(input: &str) -> Result<Vec<N>, ErrorWrapper>
where
    N: FromStr,
    ErrorWrapper: From<<N as FromStr>::Err>,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .try_fold(vec![], |mut v, r| {
            v.push(r?);
            Ok(v)
        })
}

pub fn parse_lines_with<N>(input: &str, func: fn(&str) -> Result<N, ErrorWrapper>) -> Result<Vec<N>, ErrorWrapper> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(func)
        .try_fold(vec![], |mut v, r| {
            v.push(r?);
            Ok(v)
        })
}
