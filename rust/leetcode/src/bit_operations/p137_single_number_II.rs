//! https://leetcode-cn.com/problems/single-number-ii/
//! 
//! 
//! Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.

//! Note:

//! Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

//! Example 1:

//! Input: [2,2,3,2]
//! Output: 3
//! Example 2:

//! Input: [0,1,0,1,0,1,99]
//! Output: 99
//! 

#[path = "../solution.rs"] 
mod solution;
use solution::Solution;

impl Solution {
  pub fn single_number (nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..32 {
      let mut count = 0; 
      for j in &nums {
        if j & (1 << i) == (1 << i) {
          count = count + 1;
        }
      }
      if count % 3 == 0 {
        continue;
      }
      ans = ans | (1 << i);
    }
    ans
  }
}

#[cfg(test)]
mod bit_operations_tests {
  use super::Solution;

  #[test]
  fn test_single_number () {
    assert_eq!(1, Solution::single_number(vec![1]));
    assert_eq!(3, Solution::single_number(vec![2,2,3,2]));
    assert_eq!(99, Solution::single_number(vec![0,1,0,1,0,1,99]));
  }
}
