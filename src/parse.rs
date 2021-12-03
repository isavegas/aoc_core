use std::str::FromStr;
use crate::ErrorWrapper;

pub fn parse<N>(input: &str) -> Vec<N>
where
    N: FromStr,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(str::parse)
        .filter_map(|f: Result<N, _>| match f {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect()
}

pub fn parse_with<N>(input: &str, func: fn(&str) -> Result<N, ErrorWrapper>) -> Vec<N>
where
    N: FromStr,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(func)
        .filter_map(|f: Result<N, _>| match f {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect()
}
