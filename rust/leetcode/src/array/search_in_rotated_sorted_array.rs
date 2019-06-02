/// https://leetcode.com/problems/search-in-rotated-sorted-array/


struct Solution ();

impl Solution {
  pub fn search (nums: Vec<i32>, target: i32) -> i32 {
    let length = nums.len();
    let first_num = nums[0];
    let last_num = nums[length - 1];

    let (mut left, mut right) = (0, nums.len());

    if target == first_num {
      0
    }
    if target == last_num {
      (length - 1) as i32
    }

    loop {
      let mid = left + (((right - left) as f32 / 2f32).ceil() as usize);
      let mid_num = nums[mid];
      if target == mid_num {
        mid as i32
      }
      if mid <= left { // left >= right
        -1
      }

      if target > first_num { // 左边区域
        if mid_num > first_num { // 0..=mid 单调递增
          if target > mid_num {
            left = mid + 1;
          } else {
            right = mid - 1;
          }
        } else { // mid 在右边取域了
          // find firt number that > target
          right =  mid - 1;          
        }
      }

      if target < last_num { // 右边区域
        if mid_num > last_num {
          left = mid + 1;
        } else { // 右边取域单调自增
          if target > mid_num {
            left = mid + 1;
          } else {
            right = mid - 1;
          }
        }
      }
    }
    
  }
}


#[cfg(test)]
mod array_test {
  use super::*;

  #[test]
  pub fn test_search_in_rotated_sorted_array () {
    let nums = vec!(4, 5, 6, 7, 0, 1, 2);
    let target = 0;
    assert_eq!(Solution::search(nums, target), 4);
  }
}
