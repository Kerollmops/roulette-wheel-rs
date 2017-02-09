//! A Little implementation of the roulette-wheel principle, `RouletteWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! ![Fitness proportionate selection](https://upload.wikimedia.org/wikipedia/commons/2/2a/Fitness_proportionate_selection_example.png)
//!
//! TODO implement this
//! http://www.keithschwarz.com/interesting/code/?dir=alias-method
//!
//! # Usage
//!
//! You can get this package on the
//! [crates.io/roulette-wheel](https://crates.io/crates/roulette-wheel) page.
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
//! You can `push` values onto the roulette-wheel (which will grow the wheel as needed):
//!
//! Popping values works in much the same way:
//!
//! ```
//! use roulette_wheel::RouletteWheel;
//!
//! let mut rw = RouletteWheel::new();
//!
//! rw.push(5., 'a');
//! rw.push(1., 'b');
//!
//! // you have 5 chances out of 6 to pop 'a'
//! let a_or_b = rw.pop();
//! ```

extern crate rand;

mod roulette_wheel;
pub use roulette_wheel::RouletteWheel;
