/*
Problem 001

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

use std::iter::AdditiveIterator;

pub fn problem001() -> uint {
  let max = 1000u;

  range(0u, max)
    .filter(|i| i % 3 == 0 || i % 5 == 0)
    .sum()
}

#[test]
pub fn test_problem001() {
  assert_eq!(problem001(), 233168);
}
