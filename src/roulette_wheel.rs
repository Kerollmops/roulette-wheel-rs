extern crate rand;

use std::convert::From;
use std::default::Default;
use std::iter::FromIterator;

use std::iter::repeat;
use std::collections::VecDeque;
// use std::collections::vec_deque::{ IntoIter, Iter, IterMut };
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
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Clone> From<&'a [T]> for RouletteWheel<T> {
    fn from(s: &'a [T]) -> RouletteWheel<T> {
        RouletteWheel {
            proba_sum: s.len() as f32,
            cards: VecDeque::from_iter(repeat(1.0).zip(s.iter().cloned()))
        }
    }
}

// impl<T> IntoIterator for RouletteWheel<T> {

//     type Item = (f32, T);
//     type IntoIter = IntoIter<(f32, T)>;

//     /// Creates a consuming iterator, that is, one that moves each value out of
//     /// the rouletteWheel (from start to end).
//     #[inline]
//     fn into_iter(self) -> IntoIter<(f32, T)> {
//         self.cards.into_iter()
//     }
// }

// pub struct ProbaModifier<T> {
//     container: RouletteWheel<T>, // need &'a mut !!!
//     position: usize
// }

// impl<T> ProbaModifier<T> {

//     pub fn read(&self) -> &(f32, T) {
//         &self.container.cards[self.position]
//     }

//     pub fn update(&mut self, new_proba: f32) {

//         let card = &mut self.container.cards[self.position];
//         self.container.proba_sum += card.0 - new_proba;
//         card.0 = new_proba;
//     }
// }

// impl<T> IntoIterator for RouletteWheel<T> {

//     type Item = ProbaModifier<T>;
//     type IntoIter = IntoIter<T>;

//     fn into_iter(self) -> IntoIter<T> {
//         IntoIter(ProbaModifier{
//             container: self,
//             position: 0
//         })
//     }
// }

// pub struct IntoIter<T>(ProbaModifier<T>);

// impl<T> Iterator for IntoIter<T> {

//     type Item = ProbaModifier<T>;

//     fn next(&mut self) -> Option<ProbaModifier<T>> {
//         // Some(self.0)
//         None
//     }
// }

impl<T> RouletteWheel<T> {
    // /// create a new random-wheel from vector.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let numbers: Vec<_> = (0..20).collect();
    // ///
    // /// // default probability is set to 1.0 for each element
    // /// let rw: RouletteWheel<u8> = RouletteWheel::from_vec(numbers);
    // /// ```
    // pub fn from_vec(vector: Vec<T>) -> RouletteWheel<T> {
    //     RouletteWheel {
    //         proba_sum: vector.len() as f32,
    //         cards: repeat(1.0).into_iter().zip(vector).collect()
    //     }
    // }

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
    /// let mut rw: RouletteWheel<u8> = RouletteWheel::with_capacity(numbers.len());
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
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// remove all elements in this wheel.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
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

    /// returns `true` if this wheel is empty else return `false`.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// assert_eq!(rw.is_empty(), true);
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // /// Returns an iterator over the slice.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let mut rw = RouletteWheel::new();
    // ///
    // /// rw.push(1., 'r');
    // /// rw.push(1., 'c');
    // /// rw.push(1., 'a');
    // ///
    // /// let mut iter = rw.iter();
    // ///
    // /// assert_eq!(iter.next(), Some(&(1.0, 'r')));
    // /// assert_eq!(iter.next(), Some(&(1.0, 'c')));
    // /// assert_eq!(iter.next(), Some(&(1.0, 'a')));
    // /// assert_eq!(iter.next(), None);
    // /// ```
    // pub fn iter(&self) -> Iter<(f32, T)> {
    //     self.cards.iter()
    // }

    // /// Returns an iterator that allows modifying each value.
    // /// # Example
    // ///
    // /// ```
    // /// use roulette_wheel::RouletteWheel;
    // ///
    // /// let mut rw = RouletteWheel::new();
    // ///
    // /// rw.push(1., 'r');
    // /// rw.push(1., 'c');
    // /// rw.push(1., 'a');
    // ///
    // /// for a in &mut rw.iter_mut() {
    // ///     a.1 = 'm';
    // /// }
    // ///
    // /// assert_eq!(rw.peek(), Some((1., &'m')));
    // /// ```
    // pub fn iter_mut(&mut self) -> IterMut<(f32, T)> {
    //     self.cards.iter_mut()
    // }

    /// add an element associated with a probability.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn push(&mut self, proba: f32, data: T) {
        assert!(proba > 0.0, "proba {} is lower or equal to zero!", proba);
        self.cards.push_back((proba, data));
        self.proba_sum += proba;
        if self.proba_sum.is_infinite() {
            panic!("Probability sum reached an Inf value!");
        }
    }

    /// Will recompute the probabilities sum
    /// use it when you iterate through this vector and change proba values
    pub fn compute_proba_sum(&mut self) {
        self.proba_sum = 0.0;
        for &(proba, _) in self.cards.iter() {
            assert!(proba > 0.0, "proba '{}' is lower or equal to zero!", proba);
            self.proba_sum += proba;
        }
        if self.proba_sum.is_infinite() {
            panic!("Probability sum reached an Inf value!");
        }
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
            _               => 0.
        }
    }

    /// returns a random index in self.cards.
    fn get_random_index(&self) -> Option<usize> {
        if self.is_empty() == false {
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

    /// returns a ref to the randomly peeked element with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    /// assert_eq!(rw.peek(), Some((1.0, &'r')));
    /// ```
    pub fn peek(&self) -> Option<(f32, &T)> {
        if let Some(index) = self.get_random_index() {
            if let Some(&(proba, ref data)) = self.cards.get(index) {
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// returns a mutable ref to the randomly peeked element with
    /// it's probality to be peeked.
    /// # Example
    ///
    /// ```
    /// use roulette_wheel::RouletteWheel;
    ///
    /// let mut rw = RouletteWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// match rw.peek_mut() {
    ///     Some((_, val)) => *val = 'b',
    ///     None => {}
    /// }
    ///
    /// assert_eq!(rw.peek(), Some((1.0, &'b')));
    /// ```
    pub fn peek_mut(&mut self) -> Option<(f32, &mut T)> {
        if let Some(index) = self.get_random_index() {
            if let Some(&mut (proba, ref mut data)) = self.cards.get_mut(index) {
                Some((proba, data))
            }
            else { None }
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
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.pop(), Some((1.0, 'r')));
    ///
    /// // once you pop the value, it doesn't exist anymore
    /// assert_eq!(rw.peek(), None);
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
}
