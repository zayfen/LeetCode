/// # 34. Find First and Last Positon of Element in Sorted Array
/// > href: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution ();

impl Solution {

  pub fn search_range (nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut first_pos, mut last_pos) = (-1, -1);
    let (mut left, mut right) = (0, nums.len() - 1);

    if nums.len() == 0 {
      return vec!(first_pos, last_pos);
    }

    // closures
    let get_mid_index = |left: usize, right: usize| left + ((right - left) / 2);

    // find first_position
    while left < right {
      let mid = get_mid_index(left, right);
      let mid_num = *nums.get(mid).unwrap();
      if target > mid_num {
        left = mid + 1;
      } else {
        right = mid;
      }
    }

    if *nums.get(left).unwrap() == target {
      first_pos = left as i32;
    }

    // find last_position
    right = nums.len() - 1;
    while left < right {
      let mid = get_mid_index(left, right) + 1;
      let mid_num = *nums.get(mid).unwrap();
      if target < mid_num {
        right = mid - 1;
      } else {
        left = mid;
      }
    }
    if *nums.get(right).unwrap() == target {
      last_pos = right as i32;
    }

    vec!(first_pos, last_pos)
  }
}


#[cfg(test)]
mod array_test {
  use super::*;

  #[test]
  fn test_search_range () {
    let nums = vec!(5, 7, 7, 8, 8, 10);
    assert_eq!(Solution::search_range(nums, 8), vec!(3, 4));

    let nums = vec!(5, 7, 7, 8, 8, 10);
    assert_eq!(Solution::search_range(nums, 6), vec!(-1, -1));
  }
}
