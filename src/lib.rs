extern crate rand;

#[macro_use]
extern crate rand_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
pub mod sample;
pub mod noise;
pub mod vectors;
pub mod camera;
pub mod rgb;
pub mod shapes;

pub use vectors::{Vector2, Vector3};
pub use camera::*;
pub use rgb::Rgb;
pub use shapes::*;
