/**
Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.

If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).

The replacement must be in-place and use only constant extra memory.

Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.

1,2,3 → 1,3,2
3,2,1 → 1,2,3
1,1,5 → 1,5,1

 **/


impl Solution {
  pub fn next_permutation(nums: &mut Vec<i32>) {
    let nums_len = nums.len();

    let (mut left, mut right) = (0, 0);

    let mut i = nums_len - 1;
    while i >= 1 {
      if nums[i-1] < nums[i] {
        left = i-1;
        right = i;
        break;
      }
      i-=1;
    }


    if left ==  0 && right == 0 {
      nums.reverse();
    } else {
      // find min({k | nums[k] > nums[left], 且k >= right })
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
}
