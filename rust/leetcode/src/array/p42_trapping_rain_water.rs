///! 42. Trapping Rain Water
///! https://leetcode-cn.com/problems/trapping-rain-water/

struct Solution ();
impl Solution {
  pub fn trap (height: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = height.len();
    if len == 0 {
      return result;
    }
    
    let (mut left, mut right) = (0usize, len - 1);
    let (mut left_max_height, mut right_max_height) = (0i32, 0i32);
    
    while left < right {
      if height[left] < height[right] {
        if height[left] > left_max_height {
          left_max_height = height[left];
        } else {
          result += left_max_height - height[left];
        }
        left += 1;

      } else {
        if height[right] > right_max_height {
          right_max_height = height[right];
        } else {
          result += right_max_height - height[right];
        }
        right -= 1;
      }
    }

    result
  }
}

#[cfg(test)]
mod test_array {
  use super::Solution;

  #[test]
  fn test_trap () {
    assert_eq!(Solution::trap(vec![]), 0);
    assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(Solution::trap(vec![1, 0, 1]), 1);
    assert_eq!(Solution::trap(vec![0, 1, 2, 3]), 0);
    assert_eq!(Solution::trap(vec![3, 2, 1, 0]), 0);
    assert_eq!(Solution::trap(vec![1, 1, 1]), 0);
    assert_eq!(Solution::trap(vec![5, 2, 1, 2, 1, 5]), 14);
    
  }
}
