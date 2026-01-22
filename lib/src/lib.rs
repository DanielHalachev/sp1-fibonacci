// ! This file is a library that contains the functions to compute the Fibonacci number.
// ! We implement several different variants to see how SP1 handles recursion and memoization.
// ! We wrap overflows in the recursive functions to ensure that the program terminates.

use std::collections::BTreeMap;

use alloy_sol_types::sol;

mod matrix;
use matrix::Matrix;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

/// Compute the n'th Fibonacci number (wrapping around on overflows), using iterative approach.
pub fn fibonacci_iterative(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}

/// Compute the n'th Fibonacci number (wrapping around on overflows), using recursive approach.
pub fn fibonacci_recursive(n: u32) -> (u32, u32) {
    if n == 0 {
        (0, 1)
    } else if n == 1 {
        (1, 1)
    } else {
        let (a, b) = fibonacci_recursive(n - 1);
        (b, a.wrapping_add(b))
    }
}

/// Compute the n'th Fibonacci number (wrapping around on overflows), using recursive approach with memoization.
pub fn fibonacci_recursive_with_memoization(n: u32) -> (u32, u32) {
    // NOTE: BTreeMap is to be preferred over HashMap, unless the seed is manually controlled
    // this fits the SP1 memory model better
    // using HashMap leads to excessive number of cycles increase (likely because of the hashing function)
    let mut memo = BTreeMap::new();
    fibonacci_recursive_with_memoization_helper(n, &mut memo)
}

fn fibonacci_recursive_with_memoization_helper(
    n: u32,
    memo: &mut BTreeMap<u32, (u32, u32)>,
) -> (u32, u32) {
    // Base cases - critical to prevent infinite recursion
    if n == 0 {
        return (0, 1);
    }
    if n == 1 {
        return (1, 1);
    }

    // Check memo
    if let Some((a, b)) = memo.get(&n) {
        return (*a, *b);
    }

    // Compute recursively
    let (a, b) = fibonacci_recursive_with_memoization_helper(n - 1, memo);
    let result = (b, a.wrapping_add(b));

    // Store in memo
    memo.insert(n, result);
    result
}

/// Compute the n'th Fibonacci number (wrapping around on overflows), using matrix exponentiation approach.
pub fn fibonacci_with_matrix(n: u32) -> (u32, u32) {
    let mut matrix = Matrix::identity();
    let multiplier: Matrix = Matrix::new([1, 1, 1, 0]);

    for _ in 0..n {
        matrix = matrix.multiply(&multiplier);
    }
    // After n multiplications, matrix is [[F(n+1), F(n)], [F(n), F(n-1)]]
    // We need to return (F(n), F(n+1)) to match fibonacci_iterative, which is (values[1], values[0])
    (matrix.values[1], matrix.values[0])
}
