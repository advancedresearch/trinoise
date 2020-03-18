//! # Trinoise
//! A mathematical noise pattern of 3 values based on Number Theory and Set Theory
//!
//! ### Properties
//!
//! - Assigns a value to every natural number
//! - Fixed interpretation based on aligned positions to identity map
//! - Value counts the number of successors with decreasing value
//! - Never repeats the same number twice for bases greater than `2`
//! - Repeats noise pattern after `N^N` for base `N`
//!
//! The value counts the number of successors with decrementing value.
//! One can use it to skip successors and project into 3 values:
//!
//! 0. `0`
//! 1. `base - 2`
//! 2. `base - 1`
//!
//! This is done using the `tri` function.
//!
//! The frequencies of `0`, `1` and `2` is predictable, e.g.
//! `[470, 470, 155]` for base `5`.
//!
//! The frequency of `0` and `1` are equal for bases greater than `2` (conjecture).
//!
//! The frequency of `0` or `1` divided by frequency of `2` converges rapidly
//! to `base - 2` when `base` goes to infinity (conjecture).
//!
//! ### Inspiration
//!
//! The powerset operator is important in Set Theory.
//! To generate the powerset of a finite set, one can use a binary encoding,
//! where each bit represents a membership of an object.
//!
//! One problem with the powerset operator,
//! is that it does not provide information about isomorphisms of sets
//! to themselves.
//! This extra information is desirable when studying equivalences.
//! Therefore, one would like a more "powerful" way of generating sets.
//!
//! There is a different way of generating sets that respects isomorphisms:
//!
//! - Starting with an identity map `[0, 1, 2, ..., n-1]`
//! - Modify one position at a time, e.g. `[0, 1, 2] => [0, 0, 2]`
//! - The generated discrete combinatorial space forms a groupoid
//! - Redundant members are removed through post-filtering to form subsets
//!
//! For example, `[0, 0, 0]` becomes a set which contains only `{0}`.
//!
//! Isomorphisms are also generated, e.g. `[2, 0, 1]`.
//!
//! This means that the same method construct both subsets and isomorphisms.
//! The combination of subsets and isomorphisms is interesting to study for
//! Sized Type Theory, a type theory where functions can be applied to equivalences.
//! It is believed that an equivalence can ensure the existence of a
//! partial normal path, hence not require the function to be an isomorphism.
//!
//! For example, `[0, 0, 1]` is mapped differently than `[1, 0, 0]`, but both has
//! the same set `{0, 1}`.
//! When a function maps to a smaller set, it can not be an isomorphism,
//! but in Sized Type Theory one can use `f(a ~= b) == (f(a) ~= f(b))`,
//! so this is still meaningful as
//! existence of some normal path `f[g_i->n]` where `g_i(a) == g_i(b)` and
//! `g_n(f(a)) == g_n(f(b))`.
//! Normal paths are commutative squares of functions.
//! In this case, the square commutes by definition.
//!
//! One benefit of this groupoid structure, is that it represents all possible
//! transformations of sets closed under the category of functions.
//! It is much easier to study this structure than reasoning about families of functions,
//! because in families of functions the sets are repeated many times.
//!
//! It turns out that the reachability tree with identity map as root,
//! assigns a node depth equal to `n` minus aligned positions with identity map.
//! When ordering the reachability tree, the nodes form smaller neighborhoods
//! with same node depth, which size is always `1`, `n - 1` or `n` (conjecture).
//!
//! This is because when counting upwards, the following is true:
//!
//! - For every `n` cycle, there is at least one disruption
//! - Any disruption can either collapse 2 positions, swap 1 vs 1, or collapse 1
//! - Swapping 1 vs 1 never happens twice during `n` cycle
//!
//! The order of the identity map is chosen to preserve this property.
//! If one uses `210` instead of `012` in base `3`, this property is destroyed.
//!
//! The nodes in the groupoid can naturally be encoded with numbers in base `n`.
//! In base `n`, the signature of ordered neighborhoods with same node depth
//! is encodable in base `n` by subtracting `1`, counting successors.
//! Therefore, `0`, `base - 2` or `base - 1` are the only values.

/// Counts the number of aligned positions to identity map.
///
/// The maximum number of aligned positions is equal to the base.
///
/// For example, `012` in base `3` is the identity map,
/// therefore the aligned positions are `3`.
pub fn aligned(mut v: u64, base: u8) -> u8 {
    let base = base as u64;
    let mut sum = 0;
    for i in (0..base).rev() {
        if v % base == i {sum += 1}
        v /= base;
    }
    sum
}

/// Returns the number of successors that share number of aligned positions.
///
/// This is always a number `0`, `base - 2` or `base - 1`.
///
/// This number can also be used to increase the counter.
pub fn next(mut v: u64, base: u8) -> u8 {
    let mut sum = 0;
    let mut a = aligned(v, base);
    loop {
        let b = aligned(v + 1, base);
        if a == b {sum += 1; v += 1} else {break}
        a = b;
    }
    sum
}

/// Maps `0 => 0, base - 2 => 1, base - 1 => 2`.
pub fn tri(c: u8, base: u8) -> u8 {
    if c == 0 {0} else if c == base-2 {1} else {2}
}

/// Calculates signature of successors with shared aligned positions.
pub fn signature(base: u8) -> Vec<u8> {
    let end = (base as u64).pow(base as u32);
    let mut v = 0;
    let mut r = vec![];
    // Do not include the end since it would wrap count successors.
    while v + 1 < end {
        let n = next(v, base);
        v += n as u64 + 1;
        r.push(tri(n, base));
    }
    // The end always has no successors.
    r.push(0);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(signature(2), vec![0, 0, 0, 0]);

        let base = 3;
        assert_eq!(signature(base), vec![
            1, 2, 0, 1, 0, 1, 2, 0, 1, 0, 1, 2, 0, 1, 0]);
        assert_eq!(aligned(0, base), 1);
        assert_eq!(aligned(1, base), 1);
        assert_eq!(aligned(2, base), 2);
        assert_eq!(aligned(3, base), 2);
        assert_eq!(aligned(4, base), 2);
        assert_eq!(aligned(5, base), 3);
        assert_eq!(aligned(6, base), 1);
        assert_eq!(aligned(7, base), 1);

        assert_eq!(next(0, base), 1);
        assert_eq!(next(1, base), 0);
        assert_eq!(next(2, base), 2);
        assert_eq!(next(3, base), 1);
        assert_eq!(next(4, base), 0);
        assert_eq!(next(5, base), 0);
        assert_eq!(next(6, base), 1);

        let base = 3;
        let s = signature(base);
        let mut p = [0, 0, 0];
        for i in 0..s.len() {
            p[s[i] as usize] += 1;
        }
        assert_eq!(p, [6, 6, 3]);                   // 3
        // assert_eq!(p, [44, 44, 20]);             // 4
        // assert_eq!(p, [470, 470, 155]);          // 5
        // assert_eq!(p, [6222, 6222, 1554]);       // 6
        // assert_eq!(p, [98042, 98042, 19607]);    // 7
    }
}
