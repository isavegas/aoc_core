pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> Result<String, ErrorWrapper>;
    fn part2(&self) -> Result<String, ErrorWrapper>;
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>);
}
