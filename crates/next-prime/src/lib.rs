//! A library for finding the next prime number greater than a given BigUint number
//!
//! # Example
//!
//! ```rust
//! use num_bigint::BigUint;
//! use yoshi389111_next_prime as next_prime;
//! use yoshi389111_miller_rabin::is_probable_prime;
//!
//! let n = BigUint::from(7_u32);
//! let next_prime = next_prime::find(&n, |x| is_probable_prime(x, 40));
//! assert_eq!(next_prime, BigUint::from(11_u32));
//! ```
//!
//! # References
//! - [Wheel Factorization - Wikipedia](https://en.wikipedia.org/wiki/Wheel_factorization)

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::ToPrimitive;
use once_cell::sync::Lazy;
use std::sync::Arc;

/// Structure representing a wheel sieve for prime number generation
struct WheelSieve {
    size: usize,
    residues: Vec<usize>,
    diffs: Vec<usize>,
}

/// Calculate the primorial of n (the product of the first n prime numbers)
/// Note: overflow may occur for large n
fn primorial(n: usize) -> usize {
    use yoshi389111_prime_iter as prime_iter;

    prime_iter::new::<usize>().take(n).product()
}

/// Structure representing an iterator for generating candidate prime numbers using wheel sieve
impl WheelSieve {
    /// Create a new WheelSieve instance
    fn new(wheel_prime_count: usize) -> Arc<WheelSieve> {
        let size = primorial(wheel_prime_count);
        let residues = (1..size).filter(|&x| x.gcd(&size) == 1).collect::<Vec<_>>();
        let diffs = (0..residues.len())
            .map(|i| (size + residues[(i + 1) % residues.len()] - residues[i]) % size)
            .collect::<Vec<_>>();
        Arc::new(Self {
            size,
            residues,
            diffs,
        })
    }

    /// Create an iterator starting from a given BigUint number
    fn iter(self: &Arc<Self>, start: &BigUint) -> WheelSieveIter {
        let remainder = (start % self.size).to_usize().unwrap();
        let index = (0..(self.residues.len()))
            .find(|&i| remainder <= self.residues[i])
            .unwrap_or(0);
        let next_value = start + (self.residues[index] + self.size - remainder) % self.size;
        WheelSieveIter {
            sieve: Arc::clone(self),
            index,
            next_value,
        }
    }
}

/// Iterator for generating candidate prime numbers using wheel sieve
struct WheelSieveIter {
    sieve: Arc<WheelSieve>,
    index: usize,
    next_value: BigUint,
}

impl Iterator for WheelSieveIter {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let diffs = &self.sieve.diffs;
        let next_value = &self.next_value + diffs[self.index];
        // Note: Using std::mem::replace to avoid cloning the BigUint
        let current_value = std::mem::replace(&mut self.next_value, next_value);
        self.index = (self.index + 1) % diffs.len();
        Some(current_value)
    }
}

/// The number of prime numbers used in the wheel sieve. (p4# = 2*3*5*7 = 210)
const WHEEL_PRIME_COUNT: usize = 4;

/// The wheel sieve used for generating candidate prime numbers
static WHEEL_SIEVE_210: Lazy<Arc<WheelSieve>> = Lazy::new(|| WheelSieve::new(WHEEL_PRIME_COUNT));

/// Find the next prime number greater than specified BigUint number
///
/// ## Params
///
/// - `n`: the number to find the next prime after
/// - `is_prime`: a function that checks if a given BigUint number is prime
///
/// ## Returns
///
/// - the next prime number greater than n
///
/// ## Example
///
/// ```rust
/// use num_bigint::BigUint;
/// use yoshi389111_next_prime as next_prime;
/// use yoshi389111_miller_rabin::is_probable_prime;
///
/// let n = BigUint::from(7_u32);
/// let next_prime = next_prime::find(&n, |x| is_probable_prime(x, 40));
/// assert_eq!(next_prime, BigUint::from(11_u32));
/// ```
pub fn find<F>(n: &BigUint, is_prime: F) -> BigUint
where
    F: Fn(&BigUint) -> bool,
{
    WHEEL_SIEVE_210
        .iter(&(n + 1u8))
        .find(|c| is_prime(c))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use yoshi389111_miller_rabin::is_probable_prime;

    #[test]
    fn primorial_calculates_correctly() {
        assert_eq!(primorial(1), 2);
        assert_eq!(primorial(2), 6);
        assert_eq!(primorial(3), 30);
        assert_eq!(primorial(4), 210);
    }

    #[test]
    fn wheel_sieve_30_generates_correct_candidates() {
        let sieve = WheelSieve::new(3);
        let mut iter = sieve.iter(&BigUint::from(0u32));
        let candidates: Vec<BigUint> = iter.by_ref().take(10).collect();
        let expected = vec![
            BigUint::from(1u32),
            BigUint::from(7u32),
            BigUint::from(11u32),
            BigUint::from(13u32),
            BigUint::from(17u32),
            BigUint::from(19u32),
            BigUint::from(23u32),
            BigUint::from(29u32),
            BigUint::from(31u32),
            BigUint::from(37u32),
        ];
        assert_eq!(candidates, expected);
    }

    #[test]
    fn wheel_sieve_120_generates_correct_candidates() {
        let sieve = WheelSieve::new(4);
        let mut iter = sieve.iter(&BigUint::from(0u32));
        let candidates: Vec<BigUint> = iter.by_ref().take(10).collect();
        let expected = vec![
            BigUint::from(1u32),
            BigUint::from(11u32),
            BigUint::from(13u32),
            BigUint::from(17u32),
            BigUint::from(19u32),
            BigUint::from(23u32),
            BigUint::from(29u32),
            BigUint::from(31u32),
            BigUint::from(37u32),
            BigUint::from(41u32),
        ];
        assert_eq!(candidates, expected);
    }

    #[test]
    fn find_next_prime_returns_correct_value() {
        let n = BigUint::from(9u32);
        let result = find(&n, |x| is_probable_prime(x, 40));
        assert_eq!(result, BigUint::from(11u32));
    }
}
