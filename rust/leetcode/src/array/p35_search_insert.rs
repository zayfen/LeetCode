
struct Solution ();

impl Solution {

  pub fn search_insert (nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    // if len == 0 {
    //   return 0;
    // }
    let mut left = 0;
    let mut right = len;
    while left < right {
      let mid = left + (right - left) / 2;
      if nums[mid] < target {
        left = mid + 1;
      } else {
        right = mid;
      }
    }
    return left as i32;
  }
}


#[cfg(test)]
mod array_test {
  use super::*;

  #[test]
  fn test_search_insert () {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![], 0), 0); // or -1
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
  }
}
