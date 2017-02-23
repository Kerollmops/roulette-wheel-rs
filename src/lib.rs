//! A Little implementation of the roulette-wheel principle, `RouletteWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! ![Fitness proportionate selection](https://upload.wikimedia.org/wikipedia/commons/2/2a/Fitness_proportionate_selection_example.png)
//!
//! # Usage
//!
//! # Examples
//!
//! You can explicitly create a `RouletteWheel<T>` with `new()`:
//!
//! ```
//! use roulette_wheel::RouletteWheel;
//!
//! let rw: RouletteWheel<u8> = RouletteWheel::new();
//! ```
//!
//! You can `push` values in the roulette-wheel (which will grow the wheel as needed):
//! ```
//! use roulette_wheel::RouletteWheel;
//!
//! let mut rw = RouletteWheel::new();
//!
//! rw.push(5., 'a');
//! rw.push(1., 'b');
//!
//! // iter will not consume the roulette wheel
//! for (fit, &ind) in rw.iter() {
//!    // do things with individuals here
//! }
//!
//! // into_iter consume the roulette wheel
//! for (fit, ind) in rw.into_iter() {
//!     // do things with individuals here
//! }
//!
//! ```

extern crate rand;

mod roulette_wheel;
pub use roulette_wheel::RouletteWheel;
