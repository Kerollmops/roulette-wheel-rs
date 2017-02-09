extern crate rand;

use std::iter::{FromIterator, Iterator, repeat};
use std::collections::VecDeque;
use std::collections::vec_deque::{Iter, IterMut};
use self::rand::Rng;

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
        let cap = iter.size_hint().1.unwrap_or(0);
        let mut total_fitness = 0.0;
        let mut fitnesses = Vec::with_capacity(cap);
        let mut population = Vec::with_capacity(cap);

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
    /// let rw: RouletteWheel<u8> = RouletteWheel::new();
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
    /// let numbers: Vec<_> = (0..20).collect();
    /// let rw: RouletteWheel<u8> = RouletteWheel::with_capacity(numbers.len());
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

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Ringbuf`.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw: RouletteWheel<u8> = RouletteWheel::new();
    /// rw.reserve(20);
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.fitnesses.reserve(additional);
        self.population.reserve(additional);
    }

    // /// Returns the number of elements the RouletteWheel can hold without
    // /// reallocating.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let rw: RouletteWheel<u8> = RouletteWheel::new();
    // ///
    // /// println!("actual capacity: {}", rw.capacity());
    // /// ```
    // pub fn capacity(&self) -> usize {
    //     self.population.capacity()
    // }

    /// returns the number of elements in the wheel.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// assert_eq!(rw.len(), 0);
    ///
    /// rw.push(1.0, 'r');
    /// rw.push(1.0, 'c');
    /// rw.push(1.0, 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.population.len()
    }

    /// returns `true` if empty else return `false`.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// assert_eq!(rw.is_empty(), true);
    ///
    /// rw.push(1.0, 'r');
    /// rw.push(1.0, 'c');
    /// rw.push(1.0, 'a');
    ///
    /// assert_eq!(rw.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.population.is_empty()
    }

    /// remove all elements in this wheel.
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
    ///
    /// rw.clear();
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.fitnesses.clear();
        self.population.clear();
    }

    /// add an element associated with a probability.
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
        assert!(fitness.is_finite(), "Can't push non-finite fitness {:?}", fitness);
        assert!(fitness >= 0.0, "Can't push negative fitness {:?}", fitness);

        self.fitnesses.push(fitness);
        self.population.push(individual);
        self.total_fitness += fitness;

        assert!(self.total_fitness.is_finite(), "Fitnesses sum reached a non-finite state!");
    }

    /// Returns sum of all individual fitnesses.
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

    // pub fn select_one_iter(&self) -> SelectOneIter<T> {
    //     SelectOneIter { aaa: self.population[0] }
    // }
}

pub struct SelectOneIter<T> {
    aaa: T
}

impl<T> Iterator for SelectOneIter<T> {
    type Item = (f32, T);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
