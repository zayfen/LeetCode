/**
 * 
 */

pub fn search (nums: Vec<i32>, target: i32) -> i32 {
  let length = nums.len();
  if length == 0 {
    return -1;
  }
  let (length, first_num, last_num) = (length, nums[0], nums[length - 1]);
  let (mut left, mut right) = (0, length - 1);

  if first_num == target {
    return 0 as i32;
  }

  if target == last_num {
    return (length - 1) as i32;
  }

  while left <= right {
    let mid = left + (((right - left) as f32 / 2f32).ceil() as usize);
    let mid_num = nums[mid];
    println!("nums: {:?}, (left, right): {:?}, mid: {}, mid_num: {}", nums, (left, right), mid, mid_num);
    if target == mid_num {
      return mid as i32;
    }
    if mid <= left {
      // left >= right
      return -1 as i32;
    }

    if target > first_num {
      // 左边区域
      if mid_num > first_num {
        // 0..=mid 单调递增
        if target > mid_num {
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      } else {
        // mid 在右边取域了
        // find firt number that > target
        right = mid - 1;
      }
    } else if target < last_num {
      // 右边区域
      if mid_num > last_num {
        left = mid + 1;
      } else {
        // 右边取域单调自增
        if target > mid_num {
          left = mid + 1;
        } else {
          right = mid - 1;
        }
      }
    } else {
      return -1;
    }
  }

  -1 as i32
}

#[cfg(test)]
mod array_test {
  use super::*;

  #[test]
  pub fn test_search_in_rotated_sorted_array() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(search(nums, target), 4);
    
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(search(nums, 3), -1);
  }
}
