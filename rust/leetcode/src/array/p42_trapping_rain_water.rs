///! 42. Trapping Rain Water
///! https://leetcode-cn.com/problems/trapping-rain-water/

struct Solution ();
impl Solution {
  pub fn trap (height: Vec<i32>) -> i32 {
    let mut result = 0;
    // find  v
    let len = height.len();
    if len == 0 {
      return result;
    }
    
    let mut ptr: usize = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;

    while ptr < len-1 {
      let curr_height: i32 = height[ptr];
      let next_height: i32 = height[ptr+1];

      // if next_height < curr_height, then push stack
      if next_height < curr_height {
        left = ptr;

        // skip all downstairs
        ptr += 1;
        while ptr < len-1 && height[ptr] > height[ptr+1] {
          ptr += 1;
        }
        // now ptr is on floor, scan all upstairs
        while ptr < len-1 && height[ptr] <= height[ptr+1] {
          ptr += 1;
        }

        // now ptr is on ceil
        right = ptr;

        // now calculate water in range [left, right]
        println!("left: {:?}, right: {:?}", left, right);
        let shortest_side = std::cmp::min(height[left], height[right]) as usize;
        let mut sum = std::cmp::max((right - left - 1) * shortest_side, 0);
        left += 1;
        while left < right {
          sum -= std::cmp::min(height[left] as usize, shortest_side);
          left += 1; // move forward
        }
        println!("sum: {:?}", sum);
        result += sum as i32;

        // reset context
        left = 0;
        right = 0;
        continue;
      }

      ptr += 1;
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
