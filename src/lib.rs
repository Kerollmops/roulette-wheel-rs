//! A Little implementation of the random-wheel principle, `RouletteWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! ![Fitness proportionate selection](https://upload.wikimedia.org/wikipedia/commons/2/2a/Fitness_proportionate_selection_example.png)
//!
//! # Usage
//!
//! You can get this package on the
//! [crates.io/random-wheel](https://crates.io/crates/random-wheel) page.
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
//! You can `push` values onto the random-wheel (which will grow the wheel as needed):
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

mod roulette_wheel;
pub use roulette_wheel::RouletteWheel;
