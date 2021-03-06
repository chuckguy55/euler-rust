/*
Problem 012

The sequence of triangle numbers is generated by adding the natural numbers.
So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
The first ten terms would be:
      1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:
   1: 1
   3: 1,3
   6: 1,2,3,6
  10: 1,2,5,10
  15: 1,3,5,15
  21: 1,3,7,21
  28: 1,2,4,7,14,28

We can see that 28 is the first triangle number to have over five divisors.

What is the value of the first triangle number to have over five hundred divisors?
*/

/*
28 = 2 * 2 * 7
28 has five factors: 1, 2, 7, (2*2), (7*2), (7*2*2)

28 / 2^x + 7^y == 0
28 / 2^0 + 7^0 == 0
28 / 2^1 + 7^0 == 0
28 / 2^2 + 7^0 == 0 
28 / 2^0 + 7^1 == 0
28 / 2^1 + 7^1 == 0
28 / 2^2 + 7^1 == 0

So y varies between the potential values of 7^y, since 7 can be a factor of 28 zero or once. (1, 7)
x vaires between the potential values of 2^x, which can appear as a factor zero, one or two times (1, 2, 4)

So then it's (3 choose 1) * (2 choose 1) == (3 * 2) == 6
*/

extern crate euler;
use euler::{count_factors, prime_factors};
use std::iter::MultiplicativeIterator;

fn problem012(threshold: uint) -> uint {
  let mut triangle_sum = 0u; 
  let mut count = 1u;
  loop {
    triangle_sum = triangle_sum + count;
    
    let primes_and_pows = count_factors(&prime_factors(triangle_sum));
    
    let num_of_factors = primes_and_pows
      .iter()
      .map(|(_, &power)| power+1) // we add one since 1 isn't returned as a factor (n^0 == 1)
      .product();
    
    if num_of_factors > threshold {
      return triangle_sum;
    }
    count = count + 1;
  }
}

#[test]
fn test_problem012() {
  assert_eq!(problem012(5), 28);
  assert_eq!(problem012(500), 76576500);
}
