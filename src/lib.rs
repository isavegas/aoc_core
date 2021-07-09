#![feature(option_result_contains)]

mod builder;
pub use builder::{generate_get_days, generate_get_inputs};

mod project;
pub use project::AoCProject;

pub mod parse;

mod vec3;
pub use vec3::Vec3;

mod vec2;
pub use vec2::Vec2;

pub mod math;

mod error;
pub use error::ErrorWrapper;

mod day;
pub use day::AoCDay;

// Useful for output
pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}
