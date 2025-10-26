//! An iterator that generates prime numbers sequentially using a memory-efficient algorithm.
//!
//! ## Features
//!
//! - Generates prime numbers on demand as an iterator
//! - Efficient composite number management using a hash map
//! - Supports arbitrary integer types (`usize`, `u32`, `i8`, etc.)
//! - The iterator terminates after returning all prime numbers within the range that can be handled by the specified data type.
//!
//! ## Usage
//!
//! ```rust
//! use yoshi389111_prime_iter as prime_iter;
//! for prime in prime_iter::new::<u32>().take(10) {
//!     println!("{}", prime);
//! }
//! ```
//!
//! ```rust
//! use yoshi389111_prime_iter as prime_iter;
//! let primes_under_100: Vec<u64> = prime_iter::new()
//!     .take_while(|&p| p < 100)
//!     .collect();
//! println!("{:?}", primes_under_100);
//! ```
//!
//! ## Type Constraints & Notes
//!
//! - Type `T` must satisfy `num_traits::PrimInt + Hash + Eq`.
//! - Overflow will occur if the type's maximum value is exceeded.
//! - Not thread-safe (internally uses `FxHashMap`).
//!
//! ## Performance
//!
//! - Memory usage depends on the prime range, as composite numbers are managed with a hash map.
//! - Be cautious of memory consumption when handling large values.
//!
//! ## References
//!
//! - [Sieve of Eratosthenes - Wikipedia](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)

use std::collections::BTreeMap;
use std::iter::successors;

/// Internal state for the prime number iterator using a sieve algorithm.
struct PrimeSieveIter<T: num_traits::PrimInt> {
    sieve_map: BTreeMap<T, T>,
    next_candidate: Option<T>,
}

/// Returns an iterator over prime numbers.
///
/// ## Description
///
/// The iterator starts from the smallest prime number (2) and continues
/// until the maximum value of the specified type is reached,
/// generating prime numbers using a modified Sieve of Eratosthenes algorithm.
///
/// ## Type Parameters
///
/// - `T`: The numeric type to yield (e.g., `usize`, `u32`, etc.).
///
/// ## Returns
///
/// An iterator that yields prime numbers of type `T`.
///
/// ## Example
/// ```
/// use yoshi389111_prime_iter as prime_iter;
/// let mut primes = prime_iter::new::<usize>();
/// assert_eq!(primes.next(), Some(2));
/// assert_eq!(primes.next(), Some(3));
/// assert_eq!(primes.next(), Some(5));
/// assert_eq!(primes.next(), Some(7));
/// assert_eq!(primes.next(), Some(11));
/// ```
pub fn new<T: num_traits::PrimInt>() -> impl Iterator<Item = T> {
    PrimeSieveIter {
        sieve_map: BTreeMap::new(),
        next_candidate: T::from(2u8),
    }
}

impl<T: num_traits::PrimInt> Iterator for PrimeSieveIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let two = T::from(2u8).expect("never fails");
        let curr = self.next_candidate?;
        if curr == two {
            self.next_candidate = T::from(3u8);
            return Some(two);
        }

        loop {
            let curr = self.next_candidate?;
            self.next_candidate = curr.checked_add(&two);
            if let Some(stride) = self.sieve_map.remove(&curr) {
                // composite number
                if let Some(next_composite) =
                    successors(curr.checked_add(&stride), |&n| n.checked_add(&stride))
                        .find(|n| !self.sieve_map.contains_key(n))
                {
                    self.sieve_map.insert(next_composite, stride);
                }
            } else {
                // prime number
                if let Some(next_composite) = curr.checked_mul(&curr) {
                    // `curr * 2` will not overflow because `curr * 2 < multiple`.
                    self.sieve_map.insert(next_composite, curr * two);
                }
                return Some(curr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_iterator_starting_values() {
        let mut iter = new::<usize>();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(11));
    }

    #[test]
    fn test_prime_iterator_i8() {
        let primes: Vec<i8> = new().collect();
        assert_eq!(
            primes,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127
            ]
        );
    }
}
