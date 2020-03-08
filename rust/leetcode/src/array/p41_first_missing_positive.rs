/**
41. First Missing Positive
https://leetcode-cn.com/problems/first-missing-positive/
 */

struct Solution ();

impl Solution {

  #[allow(unused)]
  pub fn first_missing_positive (nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    let mut result :usize = 1;
    // first checkout that 1 included in nums
    // all numbers less than 1 be 1
    let mut _1_contained = nums.contains(&1);
    if !_1_contained {
      return result as i32;
    }

    let mut nums: Vec<i32> = nums.into_iter().map(|num| {
      if num < 1 || num as usize > len {
        1
      } else {
        num
      }
    }).collect();

    
    for i in 0..len {
      let index = nums[i].abs() - 1; // as index of nums
      nums[index as usize] = if nums[index as usize] < 0 {
        nums[index as usize]
      } else {
        -nums[index as usize]
      }
    }

    for (index, val) in nums.iter().enumerate() {
      if *val > 0 {
        result = index + 1;
        break;
      }
      // the last element < 0, then result is length+1
      if index == len - 1 {
        result = len + 1;
      }
    }

    result as i32
  }

}

#[cfg(test)]
mod test_array {
  use super::Solution;

  #[test]
  fn test_first_missing_positive () {
    
    assert_eq!(Solution::first_missing_positive(vec![-1, -1, 2, 0, 4, -2, 1]), 3);
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3]), 4);
    assert_eq!(Solution::first_missing_positive(vec![-1, -2, -3]), 1);
  }
}





