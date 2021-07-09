pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self, input: &str) -> Result<String, crate::ErrorWrapper>;
    fn part2(&self, input: &str) -> Result<String, crate::ErrorWrapper>;
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
}
