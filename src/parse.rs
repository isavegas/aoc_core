use std::str::FromStr;

pub fn numbers<N>(input: &str) -> Vec<N>
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
