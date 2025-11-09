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

mod wheel_sieve;
use crate::wheel_sieve::WheelSieve;
use num_bigint::BigUint;
use once_cell::sync::Lazy;
use std::sync::Arc;

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
    fn find_next_prime_returns_correct_value() {
        let n = BigUint::from(9u32);
        let result = find(&n, |x| is_probable_prime(x, 40));
        assert_eq!(result, BigUint::from(11u32));
    }
}
