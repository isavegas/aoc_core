#![feature(option_result_contains)]

mod builder;
pub use builder::generate_get_days;

mod vec3;
pub use vec3::Vec3;

mod vec2;
pub use vec2::Vec2;

pub mod math;

mod error;
pub use error::ErrorWrapper;

pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> Result<String, ErrorWrapper>;
    fn part2(&self) -> Result<String, ErrorWrapper>;
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>);
}

// Useful for output
pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}
