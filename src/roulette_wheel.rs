extern crate rand;

use std::convert::From;
use std::default::Default;
use std::iter::{FromIterator, Iterator, repeat};
use std::collections::VecDeque;
use std::collections::vec_deque::Iter;
use self::rand::Rng;

/// a little implementation of a random-wheel.
pub struct RouletteWheel<T> {
    /// the sum of all probabilities in this wheel.
    proba_sum: f32,
    /// all the (probability, data) in a linked-list to pop easily.
    cards: VecDeque<(f32, T)>
}

impl<T: Clone> Clone for RouletteWheel<T> {
    fn clone(&self) -> RouletteWheel<T> {
        RouletteWheel {
            proba_sum: self.proba_sum,
            cards: self.cards.clone()
        }
    }
}

impl<T> Default for RouletteWheel<T> {
    fn default() -> RouletteWheel<T> {
        RouletteWheel {
            proba_sum: 0.0,
            cards: VecDeque::new()
        }
    }
}

// impl<'a, T: Clone> From<&'a [T]> for RouletteWheel<T> {
//     fn from(s: &'a [T]) -> RouletteWheel<T> {
//         RouletteWheel {
//             proba_sum: s.len() as f32,
//             cards: VecDeque::from_iter(repeat(1.0).zip(s.iter().cloned()))
//         }
//     }
// }

impl<'a, T: Clone> From<&'a [(f32, T)]> for RouletteWheel<T> {
    fn from(s: &'a [(f32, T)]) -> RouletteWheel<T> {
        for &(proba, _) in s {
            assert!(proba >= 0.0, "probability '{}' is lower to zero!", proba);
        }
        let proba_sum = s.iter().fold(0.0, |acc, &(proba, _)| acc + proba);
        assert!(proba_sum > 0.0, "probabilities must not be all zero!");
        RouletteWheel {
            proba_sum: proba_sum,
            cards: VecDeque::from_iter(s.iter().cloned())
        }
    }
}

pub struct PopIter<'a, T: 'a> {
    container: &'a mut RouletteWheel<T>
}

impl<'a, T> Iterator for PopIter<'a, T> {
    type Item = (f32, T);

    fn next(&mut self) -> Option<(f32, T)> {
        self.container.pop()
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
            proba_sum: 0.0,
            cards: VecDeque::new()
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
    pub fn with_capacity(n: usize) -> RouletteWheel<T> {
        RouletteWheel {
            proba_sum: 0.0,
            cards: VecDeque::with_capacity(n)
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
        self.cards.reserve(additional);
    }

    /// Returns the number of elements the RouletteWheel can hold without
    /// reallocating.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let rw: RouletteWheel<u8> = RouletteWheel::new();
    ///
    /// println!("actual capacity: {}", rw.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.cards.capacity()
    }

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
        self.cards.len()
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
        self.len() == 0
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
        self.cards.clear()
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
    pub fn push(&mut self, proba: f32, data: T) {
        assert!(proba >= 0.0, "proba {} is lower to zero!", proba);
        self.cards.push_back((proba, data));
        self.proba_sum += proba;
        assert!(!self.proba_sum.is_infinite(), "Probability sum reached an Inf value!");
    }

    /// Will recompute the probabilities sum
    /// use it when you iterate through this vector and change proba values
    /// TODO check if proba become Inf too
    pub fn update_proba_sum(&mut self) {
        self.proba_sum = 0.0;
        for &(proba, _) in self.cards.iter() {
            assert!(proba >= 0.0, "probability '{}' is lower to zero!", proba);
            self.proba_sum += proba;
        }
        assert!(!self.proba_sum.is_infinite(), "Probability sum reached an Inf value!");
    }

    /// returns total of luck you pushed.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1.5, 'r');
    /// rw.push(2., 'c');
    /// rw.push(3., 'a');
    ///
    /// assert_eq!(rw.proba_sum(), 6.5);
    /// ```
    pub fn proba_sum(&self) -> f32 {
        self.proba_sum
    }

    /// returns a random distance to browser between 0 and the probabilities sum.
    fn gen_random_dist(&self) -> f32 {
        match self.proba_sum {
            sum if sum > 0. => rand::thread_rng().gen_range(0., sum),
            _ => 0.
        }
    }

    /// returns a random index in self.cards.
    fn get_random_index(&self) -> Option<usize> {
        if self.is_empty() == false && self.proba_sum() > 0.0 {
            let mut dist = self.gen_random_dist();
            for (id, &(ref proba, _)) in self.cards.iter().enumerate() {
                dist -= *proba;
                if dist <= 0. {
                    return Some(id);
                }
            }
            None
        }
        else { None }
    }

    /// removes a randomly peeked element and return it with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1.0, 'r');
    ///
    /// assert_eq!(rw.pop(), Some((1.0, 'r')));
    ///
    /// // once you pop the value, it doesn't exist anymore
    /// // assert_eq!(rw.peek(), None);
    /// assert_eq!(rw.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<(f32, T)> {
        if let Some(index) = self.get_random_index() {
            if let Some((proba, data)) = self.cards.remove(index) {
                self.proba_sum -= proba;
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// Returns an iterator over the slice, giving &(proba, elem).
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
    /// let mut iter = rw.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1.0, 'r')));
    /// assert_eq!(iter.next(), Some(&(1.0, 'c')));
    /// assert_eq!(iter.next(), Some(&(1.0, 'a')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<(f32, T)> {
        self.cards.iter()
    }

    /// Returns an iterator that pop a random element.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// // let proba_slice: &[(f32, char)] = &[(1.0, 'a'), (3.0, 'b'), (0.5, 'c')];
    /// // let mut rw: RouletteWheel<char> = RouletteWheel::from(proba_slice);
    ///
    /// let proba_slice: &[(_, char)] = &[(1.0, 'a'), (3.0, 'b'), (0.5, 'c')];
    /// let mut rw = RouletteWheel::from(proba_slice);
    ///
    /// for (proba, elem) in rw.pop_iter() {
    ///     println!("{} : {}", proba, elem);
    /// }
    /// assert!(rw.is_empty());
    /// ```
    pub fn pop_iter(&mut self) -> PopIter<T> {
        PopIter { container: self }
    }
}

    // /// Returns a mutable iterator over the slice, giving &mut (proba, elem).
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let mut rw = RouletteWheel::new();
    // ///
    // /// rw.push(1.0, 'r');
    // /// rw.push(1.0, 'c');
    // /// rw.push(1.0, 'a');
    // ///
    // /// let mut iter = rw.iter();
    // ///
    // /// assert_eq!(iter.next(), Some(&(1.0, 'r')));
    // /// assert_eq!(iter.next(), Some(&(1.0, 'c')));
    // /// assert_eq!(iter.next(), Some(&(1.0, 'a')));
    // /// assert_eq!(iter.next(), None);
    // /// ```
    // pub fn iter_mut(&mut self) -> IterMut<(f32, T)> {
    //     self.cards.iter_mut()
    // }

    // /// returns a ref to the randomly peeked element with
    // /// it's probality to be peeked.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let mut rw = RouletteWheel::new();
    // ///
    // /// rw.push(1.0, 'r');
    // ///
    // /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    // /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    // /// ```
    // pub fn peek(&self) -> Option<(f32, &T)> {
    //     if let Some(index) = self.get_random_index() {
    //         if let Some(&(proba, ref data)) = self.cards.get(index) {
    //             Some((proba, data))
    //         }
    //         else { None }
    //     }
    //     else { None }
    // }

    // /// returns a mutable ref to the randomly peeked element with
    // /// it's probality to be peeked.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let mut rw = RouletteWheel::new();
    // ///
    // /// rw.push(1.0, 'r');
    // ///
    // /// match rw.peek_mut() {
    // ///     Some((_, val)) => *val = 'b',
    // ///     None => {}
    // /// }
    // ///
    // /// assert_eq!(rw.peek(), Some((1.0, &'b')));
    // /// ```
    // pub fn peek_mut(&mut self) -> Option<(f32, &mut T)> {
    //     if let Some(index) = self.get_random_index() {
    //         if let Some(&mut (proba, ref mut data)) = self.cards.get_mut(index) {
    //             Some((proba, data))
    //         }
    //         else { None }
    //     }
    //     else { None }
    // }
