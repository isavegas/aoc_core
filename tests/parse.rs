use aoc_core::{parse, parse_with, parse_lines, parse_lines_with, ErrorWrapper};

#[test]
fn parse_simple() {
    const DESC: &str = "Should parse delimited strings for types implementing FromStr";
    let expected: Vec<usize> = vec![0, 1, 2, 3, 4];
    let output = parse::<usize>("0,1,2,3,4", ",");
    assert!(output.is_ok(), "{}", DESC);
    assert_eq!(output.unwrap(), expected, "{}", DESC);
}

#[test]
fn parse_with_simple() {
    const DESC: &str = "Should parse delimited strings for types implementing FromStr";
    let input = "0,1,2,3,4";
    let expected: Vec<usize> = vec![0, 1, 2, 3, 4];
    let output = parse_with::<usize>(input, ",",
        |s| s.parse::<usize>().map_err(|e| ErrorWrapper::Wrapped(Box::new(e)))
    );
    assert!(output.is_ok(), "{}", DESC);
    assert_eq!(output.unwrap(), expected, "{}", DESC);
}

#[test]
fn parse_lines_simple() {
    const DESC: &str = "Should parse delimited strings for types implementing FromStr";
    let input = "0\n1\n2\n3\n4";
    let expected: Vec<usize> = vec![0, 1, 2, 3, 4];
    let output = parse_lines::<usize>(input);
    assert!(output.is_ok(), "{}", DESC);
    assert_eq!(output.unwrap(), expected, "{}", DESC);
}

#[test]
fn parse_lines_with_simple() {
    const DESC: &str = "Should parse delimited strings for types implementing FromStr";
    let input = "0\n1\n2\n3\n4";
    let expected: Vec<usize> = vec![0, 1, 2, 3, 4];
    let output = parse_lines_with::<usize>(input,
        |s| s.parse::<usize>().map_err(|e| ErrorWrapper::Wrapped(Box::new(e)))
    );
    assert!(output.is_ok(), "{}", DESC);
    assert_eq!(output.unwrap(), expected, "{}", DESC);
}
