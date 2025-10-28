//! Miller-Rabin primality test
//!
//! This crate provides an implementation of the Miller-Rabin probabilistic primality test,
//! which is widely used for efficiently checking whether large numbers are prime.
//!
//! ## Features
//!
//! - Probabilistic: can quickly identify composite numbers, and declares numbers as "probably prime" with a configurable error probability
//! - Supports both `BigUint` and `BigInt` types
//!
//! ## Usage
//!
//! ```rust
//! use num_bigint::BigUint;
//! use rand::rngs::OsRng;
//! use yoshi389111_miller_rabin::is_probable_prime;
//!
//! let w = BigUint::from(389_111_u64);
//! let is_prime = is_probable_prime(&w, 40);
//! ```
//!
//!! ## Reference
//!
//! - <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf>
//!   - Appendix B.3.1 Miller-Rabin Probabilistic Primality Test
//!   - Appendix B.3, Table B.1 Minimum number of rounds of M-R testing
//!     when generating primes for use in RSA Digital Signatures

use num_bigint::{BigUint, RandBigInt};
use once_cell::sync::Lazy;

/// A list of small prime numbers for trial division
static PRIMES: Lazy<Vec<BigUint>> = Lazy::new(|| {
    yoshi389111_prime_iter::new::<usize>()
        .take_while(|p| *p < 10_000_usize)
        .map(BigUint::from)
        .collect()
});

/// Zero constant for BigUint
static ZERO: Lazy<BigUint> = Lazy::new(|| BigUint::from(0u8));
/// One constant for BigUint
static ONE: Lazy<BigUint> = Lazy::new(|| BigUint::from(1u8));
/// Two constant for BigUint
static TWO: Lazy<BigUint> = Lazy::new(|| BigUint::from(2u8));

/// Check if a BigUint number is probably prime using trial division and Miller-Rabin test
///
/// ## Params
///
/// - `w`: the number to be tested for primality
/// - `iter`: number of iterations
/// - `rng`: random number generator
///
/// ## Returns
///
/// - `true` if `w` is probably prime
/// - `false` if `w` is definitely composite
///
/// ## Example
///
/// ```rust
/// use num_bigint::BigUint;
/// use rand::rngs::OsRng;
/// use yoshi389111_miller_rabin::is_probable_prime_with_rng;
///
/// let w = BigUint::from(389_111_u64);
/// let mut rng = OsRng;
/// let is_prime = is_probable_prime_with_rng(&w, 40, &mut rng);
/// ```
pub fn is_probable_prime_with_rng<R: rand::Rng + ?Sized>(
    w: &BigUint,
    iter: usize,
    rng: &mut R,
) -> bool {
    if w <= &ONE {
        return false;
    }

    // trial division by small primes
    for p in PRIMES.iter() {
        if w == p {
            return true;
        }
        if w % p == *ZERO {
            return false;
        }
    }

    let w_minus_1 = w - 1u8;

    // step 1.
    let a = w_minus_1.trailing_zeros().expect("always w >= 2");
    // step 2.
    let m = &w_minus_1 >> a;
    // step 4.
    for _ in 0..iter {
        // step 4.1 - 4.2
        let b = rng.gen_biguint_range(&TWO, &w_minus_1);
        // step 4.3
        let z = b.modpow(&m, w);
        // step 4.4
        if z == *ONE || z == w_minus_1 {
            continue;
        }
        // step 4.5
        let result = (1..a)
            .scan(z, |z, _| Some(z.modpow(&TWO, w))) // step 4.5.1
            .find(|z| z == &w_minus_1 || *z == *ONE);
        match result {
            Some(z) if z == w_minus_1 => continue, // step 4.7
            Some(_) => return false,               // step 4.6
            None => return false,                  // step 4.6
        };
    }
    true // step 5.
}

/// Check if a BigUint number is probably prime using Miller-Rabin test with OS random number generator
///
/// ## Params
///
/// - `w`: the number to be tested for primality
/// - `iter`: number of iterations
///
/// ## Returns
///
/// - `true` if `w` is probably prime
/// - `false` if `w` is definitely composite
///
/// ## Example
///
/// ```rust
/// use num_bigint::BigUint;
/// use yoshi389111_miller_rabin::is_probable_prime;
///
/// let w = BigUint::from(389_111_u64);
/// let is_prime = is_probable_prime(&w, 40);
/// ```
pub fn is_probable_prime(w: &BigUint, iter: usize) -> bool {
    is_probable_prime_with_rng(w, iter, &mut rand::rngs::OsRng)
}

/// Check if a BigInt number is probably prime using Miller-Rabin test with OS random number generator
///
/// ## Notes
///
/// Negative numbers and zero cannot be prime by definition, so this function always returns `false` for such inputs.
///
/// ## Params
///
/// - `w`: the number to be tested for primality
/// - `iter`: number of iterations
///
/// ## Returns
///
/// - `true` if `w` is probably prime
/// - `false` if `w` is definitely composite
///
/// ## Example
///
/// ```rust
/// use num_bigint::BigInt;
/// use yoshi389111_miller_rabin::is_probable_prime_bigint;
///
/// let w = BigInt::from(389_111_i64);
/// let is_prime = is_probable_prime_bigint(&w, 40);
/// ```
pub fn is_probable_prime_bigint(w: &num_bigint::BigInt, iter: usize) -> bool {
    match w.to_biguint() {
        Some(u) => is_probable_prime(&u, iter),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_probable_prime_with_prime() {
        let prime = BigUint::from(18446744073709551557_u64);
        assert!(is_probable_prime(&prime, 40));
    }

    #[test]
    fn test_is_probable_prime_with_composite() {
        let composite = BigUint::from(389111_u64 * 389111_u64);
        assert!(!is_probable_prime(&composite, 40));
    }

    #[test]
    fn test_is_probable_prime_with_small_numbers() {
        assert!(!is_probable_prime(&BigUint::from(0u8), 40));
        assert!(!is_probable_prime(&BigUint::from(1u8), 40));
        assert!(is_probable_prime(&BigUint::from(2u8), 40));
        assert!(is_probable_prime(&BigUint::from(3u8), 40));
        assert!(!is_probable_prime(&BigUint::from(4u8), 40));
    }
}
