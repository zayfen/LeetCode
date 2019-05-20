/// 31. Next Permutation
/// https://leetcode.com/problems/next-permutation/

struct Solution ();

impl Solution {

  pub fn next_permutation(nums: &mut Vec<i32>) {
    println!("{:?}", nums);

    let nums_len = nums.len();

    let (mut left, mut right) = (0, 0);
    let mut i = nums_len - 1;
    while i >= 1 {
      if nums[i-1] < nums[i] {
        left = i-1;
        right = i;
        break;
      }
      i -= 1;
    }

    if left ==  0 && right == 0 {
      nums.reverse();
      return;
    }

    // find min({k | nums[k] > nums[left] 且 k >= right })
    let mut k = right;
    let left_num = nums[left];
    for i in right..nums_len {
      if nums[i] > left_num && nums[i] <= nums[k] {
        k = i
      }
    }

    nums[left] = nums[k];
    nums[k] = left_num;

    // reverse nums[k+1..]
    println!("left: {:?}; right: {:?}", left, right);

    let mut step = 1;
    for i in right..nums_len {
      let (l, r) = (i, nums_len - step);
      if l >= r {
        return;
      }

      let tmp = nums[l];
      nums[l] = nums[r];
      nums[r] = tmp;
      step += 1;
    }

  }
}


#[cfg(test)]
mod array_test {
  use super::*;

  #[test]
  fn next_permutation_test () {
    let mut vec = vec!(1, 2, 3);
    Solution::next_permutation(&mut vec);
    assert_eq!(vec!(1, 3, 2), vec);

    let mut vec2 = vec!(3, 2, 1);
    Solution::next_permutation(&mut vec2);
    assert_eq!(vec!(1, 2, 3), vec2);

    let mut vec3 = vec!(1, 1, 5);
    Solution::next_permutation(&mut vec3);
    assert_eq!(vec!(1, 5, 1), vec3);

    let mut vec4 = vec!(1, 3, 2);
    Solution::next_permutation(&mut vec4);
    assert_eq!(vec!(2, 1, 3), vec4);

    let mut vec5 = vec!(2, 3, 1, 3, 3);
    Solution::next_permutation(&mut vec5);
    assert_eq!(vec!(2, 3, 3, 1, 3), vec5);

  }
}
