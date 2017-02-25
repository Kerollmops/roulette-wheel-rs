//! A Little implementation of the roulette-wheel principle, `RouletteWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! ![Fitness proportionate selection](https://upload.wikimedia.org/wikipedia/commons/2/2a/Fitness_proportionate_selection_example.png)
//!
//! # Examples usages
//!
//! You can explicitly create a `RouletteWheel<T>` with `new()`:
//!
//! ```
//! use roulette_wheel::RouletteWheel;
//!
//! let rw = RouletteWheel::<u8>::new();
//! ```
//!
//! You can `push` values in the roulette-wheel (which will grow the wheel as needed):
//!
//! ```
//! use roulette_wheel::RouletteWheel;
//! use std::iter;
//!
//! let rw: RouletteWheel<_> = iter::repeat(1.0).zip(1..5).collect();
//!
//! // .select_iter() will not consume the roulette wheel
//! for (fit, &ind) in rw.select_iter() {
//!    // do things with individuals here
//! }
//!
//! // .into_iter() consume the roulette wheel
//! for (fit, ind) in rw {
//!     // do things with individuals here
//! }
//!
//! ```

extern crate rand;

use std::iter::{FromIterator, Iterator, IntoIterator};
use rand::{Rng, ThreadRng, thread_rng};
use rand::distributions::{Range, IndependentSample};

/// A roulette-wheel container
pub struct RouletteWheel<T> {
    total_fitness: f32,
    fitnesses: Vec<f32>,
    population: Vec<T>
}

impl<T: Clone> Clone for RouletteWheel<T> {
    fn clone(&self) -> RouletteWheel<T> {
        RouletteWheel {
            total_fitness: self.total_fitness,
            fitnesses: self.fitnesses.clone(),
            population: self.population.clone()
        }
    }
}

impl<T> FromIterator<(f32, T)> for RouletteWheel<T> {
    fn from_iter<A>(iter: A) -> Self where A: IntoIterator<Item=(f32, T)> {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        let mut total_fitness = 0.0;
        let mut fitnesses = Vec::with_capacity(lower);
        let mut population = Vec::with_capacity(lower);

        for (fitness, individual) in iter {
            total_fitness += fitness;
            fitnesses.push(fitness);
            population.push(individual);
        }

        RouletteWheel {
            total_fitness: total_fitness,
            fitnesses: fitnesses,
            population: population
        }

        // #![feature(field_init_shorthand)]
        // RouletteWheel { total_fitness, fitnesses, population }
    }
}

impl<T> RouletteWheel<T> {
    /// create a new empty random-wheel.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let rw = RouletteWheel::<u8>::new();
    /// ```
    pub fn new() -> RouletteWheel<T> {
        RouletteWheel {
            total_fitness: 0.0,
            fitnesses: Vec::new(),
            population: Vec::new()
        }
    }

    /// Creates an empty RouletteWheel with space for at least n elements.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let rw = RouletteWheel::<u8>::with_capacity(15);
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn with_capacity(cap: usize) -> RouletteWheel<T> {
        RouletteWheel {
            total_fitness: 0.0,
            fitnesses: Vec::with_capacity(cap),
            population: Vec::with_capacity(cap)
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::<u8>::new();
    /// rw.reserve(20);
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.fitnesses.reserve(additional);
        self.population.reserve(additional);
    }

    /// Returns the number of elements in the wheel.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let rw: RouletteWheel<_> = [(0.1, 10), (0.2, 15), (0.5, 20)].iter().cloned().collect();
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.population.len()
    }

    /// Returns `true` if empty else return `false`.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let empty_rw = RouletteWheel::<u8>::new();
    ///
    /// assert_eq!(empty_rw.is_empty(), true);
    ///
    /// let non_empty_rw: RouletteWheel<_> = [(0.1, 10), (0.2, 15), (0.5, 20)].iter().cloned().collect();
    ///
    /// assert_eq!(non_empty_rw.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.population.is_empty()
    }

    /// Remove all elements in this wheel.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw: RouletteWheel<_> = [(0.1, 10), (0.2, 15), (0.5, 20)].iter().cloned().collect();
    ///
    /// assert_eq!(rw.len(), 3);
    ///
    /// rw.clear();
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.fitnesses.clear();
        self.population.clear();
    }

    /// Add an element associated with a probability.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1.0, 'r');
    /// rw.push(1.0, 'c');
    /// rw.push(1.0, 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn push(&mut self, fitness: f32, individual: T) {
        assert!(fitness >= 0.0, "Can't push the less than zero fitness: {:?}", fitness);
        assert!((self.total_fitness + fitness).is_finite(), "Fitnesses sum reached a non-finite value!");
        unsafe { self.unchecked_push(fitness, individual) }
    }

    /// Add an element associated with a probability.
    /// This unsafe function doesn't check for total fitness overflow
    /// nether fitness positivity.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// unsafe { rw.unchecked_push(1.0, 'r') };
    /// unsafe { rw.unchecked_push(1.0, 'c') };
    /// unsafe { rw.unchecked_push(1.0, 'a') };
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub unsafe fn unchecked_push(&mut self, fitness: f32, individual: T) {
        self.total_fitness += fitness;
        self.fitnesses.push(fitness);
        self.population.push(individual);
    }

    /// Returns the sum of all individual fitnesses.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(3.0, 'r');
    /// rw.push(2.0, 'c');
    /// rw.push(1.5, 'a');
    ///
    /// assert_eq!(rw.total_fitness(), 6.5);
    /// ```
    pub fn total_fitness(&self) -> f32 {
        self.total_fitness
    }

    pub fn select_iter(&self) -> SelectIter<ThreadRng, T> {
        SelectIter::<ThreadRng, _>::new(&self)
    }
}

pub struct SelectIter<'a, R: Rng, T: 'a> {
    distribution_range: Range<f32>,
    rng: R,
    total_fitness: f32,
    fitnesses_ids: Vec<(usize, f32)>,
    roulette_wheel: &'a RouletteWheel<T>
}

impl<'a, R: Rng, T> SelectIter<'a, R, T> {
    pub fn new(roulette_wheel: &'a RouletteWheel<T>) -> SelectIter<'a, ThreadRng, T> {
        SelectIter::from_rng(roulette_wheel, thread_rng())
    }

    pub fn from_rng(roulette_wheel: &'a RouletteWheel<T>, rng: R) -> SelectIter<'a, R, T> {
        SelectIter {
            distribution_range: Range::new(0.0, 1.0),
            rng: rng,
            total_fitness: roulette_wheel.total_fitness,
            fitnesses_ids: roulette_wheel.fitnesses.iter().cloned().enumerate().collect(),
            roulette_wheel: roulette_wheel
        }
    }
}

impl<'a, R: Rng, T: 'a> Iterator for SelectIter<'a, R, T> {
    type Item = (f32, &'a T);

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.fitnesses_ids.len(), Some(self.fitnesses_ids.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fitnesses_ids.is_empty() {
            let sample = self.distribution_range.ind_sample(&mut self.rng);
            let mut selection = sample * self.total_fitness;
            let index = self.fitnesses_ids.iter().position(|&(_, fit)| {
                            selection -= fit;
                            selection <= 0.0
                        }).unwrap();
            let (index, fitness) = self.fitnesses_ids.swap_remove(index);
            self.total_fitness -= fitness;
            Some((fitness, &self.roulette_wheel.population[index]))
        }
        else { None }
    }
}

impl<T> IntoIterator for RouletteWheel<T> {
    type Item = (f32, T);
    type IntoIter = IntoSelectIter<ThreadRng, T>;

    fn into_iter(self) -> IntoSelectIter<ThreadRng, T> {
        IntoSelectIter::<ThreadRng, _>::new(self)
    }
}

pub struct IntoSelectIter<R: Rng, T> {
    distribution_range: Range<f32>,
    rng: R,
    total_fitness: f32,
    fitnesses: Vec<f32>,
    population: Vec<T>
}

impl<R: Rng, T> IntoSelectIter<R, T> {
    pub fn new(roulette_wheel: RouletteWheel<T>) -> IntoSelectIter<ThreadRng, T> {
        IntoSelectIter::from_rng(roulette_wheel, thread_rng())
    }

    pub fn from_rng(roulette_wheel: RouletteWheel<T>, rng: R) -> IntoSelectIter<R, T> {
        IntoSelectIter {
            distribution_range: Range::new(0.0, 1.0),
            rng: rng,
            total_fitness: roulette_wheel.total_fitness,
            fitnesses: roulette_wheel.fitnesses,
            population: roulette_wheel.population
        }
    }
}

impl<R: Rng, T> Iterator for IntoSelectIter<R, T> {
    type Item = (f32, T);

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.fitnesses.len(), Some(self.fitnesses.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fitnesses.is_empty() {
            let sample = self.distribution_range.ind_sample(&mut self.rng);
            let mut selection = sample * self.total_fitness;
            let index = self.fitnesses.iter().position(|fit| {
                            selection -= *fit;
                            selection <= 0.0
                        }).unwrap();
            let fitness = self.fitnesses.swap_remove(index);
            let individual = self.population.swap_remove(index);
            self.total_fitness -= fitness;
            Some((fitness, individual))
        }
        else { None }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::StdRng;
    use {RouletteWheel, SelectIter, IntoSelectIter};

    const SEED: [usize; 4] = [4, 2, 42, 4242];

    #[test]
    fn test_select_iter_seeded() {
        let rng = StdRng::from_seed(&SEED);

        let fitnesses = [0.1, 0.2, 0.3, 0.4, 0.5];
        let fitnesses = fitnesses.iter().cloned();
        let population = 15..20;
        let rw: RouletteWheel<_> = fitnesses.zip(population).collect();

        let mut iter = SelectIter::from_rng(&rw, rng);

        assert_eq!(iter.next(), Some((0.5, &19)));
        assert_eq!(iter.next(), Some((0.3, &17)));
        assert_eq!(iter.next(), Some((0.4, &18)));
        assert_eq!(iter.next(), Some((0.2, &16)));
        assert_eq!(iter.next(), Some((0.1, &15)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_select_iter_seeded() {
        let rng = StdRng::from_seed(&SEED);

        let fitnesses = [0.1, 0.2, 0.3, 0.4, 0.5];
        let fitnesses = fitnesses.iter().cloned();
        let population = 15..20;
        let rw: RouletteWheel<_> = fitnesses.zip(population).collect();

        let mut iter = IntoSelectIter::from_rng(rw, rng);

        assert_eq!(iter.next(), Some((0.5, 19)));
        assert_eq!(iter.next(), Some((0.3, 17)));
        assert_eq!(iter.next(), Some((0.4, 18)));
        assert_eq!(iter.next(), Some((0.2, 16)));
        assert_eq!(iter.next(), Some((0.1, 15)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_len() {
        let mut rw = RouletteWheel::<u8>::new();

        assert_eq!(rw.len(), 0);

        rw.push(0.1, 1);
        rw.push(0.1, 1);
        rw.push(0.1, 1);
        rw.push(0.1, 1);

        assert_eq!(rw.len(), 4);
    }
}
