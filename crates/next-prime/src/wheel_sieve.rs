use itertools::Itertools;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::ToPrimitive;
use std::sync::Arc;

/// Structure representing a wheel sieve for prime number generation
pub(crate) struct WheelSieve {
    size: usize,
    residues: Vec<usize>,
    diffs: Vec<usize>,
}

/// Calculate the primorial of n (the product of the first n prime numbers)
/// Note: overflow may occur for large n
fn primorial(n: usize) -> usize {
    use yoshi389111_prime_iter as prime_iter;

    prime_iter::new::<usize>()
        .take(n)
        .fold(1, |acc, p| acc.checked_mul(p).expect("overflow"))
}

/// Structure representing an iterator for generating candidate prime numbers using wheel sieve
impl WheelSieve {
    /// Create a new WheelSieve instance
    pub(crate) fn new(wheel_prime_count: usize) -> Arc<WheelSieve> {
        let size = primorial(wheel_prime_count);
        let residues = (1..size).filter(|&x| x.gcd(&size) == 1).collect::<Vec<_>>();
        let diffs = residues
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| (size + b - a) % size)
            .collect::<Vec<_>>();
        Arc::new(Self {
            size,
            residues,
            diffs,
        })
    }

    /// Create an iterator starting from a given BigUint number
    pub(crate) fn iter(self: &Arc<Self>, start: &BigUint) -> impl Iterator<Item = BigUint> + use<> {
        let remainder = (start % self.size).to_usize().unwrap();
        let index = self
            .residues
            .iter()
            .position(|&i| remainder <= i)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
