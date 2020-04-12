// https://leetcode-cn.com/problems/kth-largest-element-in-an-array/

pub fn parent (i: usize) -> usize {
  if i == 0 {
    return i;
  }
  (i - 1) / 2
}

pub fn left_child (i: usize) -> usize {
  i * 2 + 1
}

pub fn right_child (i: usize) -> usize {
  i * 2 + 2
}

pub fn switch (nums: &mut Vec<i32>, left: usize, right: usize) {
  let tmp = nums[left];
  nums[left] = nums[right];
  nums[right] = tmp;
}

pub fn heapify (nums: &mut Vec<i32>, heap_size: usize, i: usize) {
  let mut largest: usize = i;

  let left = left_child(i);
  if left < heap_size && nums[left] > nums[largest] {
    largest = left;
  }

  let right = right_child(i);
  if right < heap_size && nums[right] > nums[largest] {
    largest = right;
  }

  if largest != i {
    // switch i and largest
    switch(nums, largest, i);
    heapify(nums, heap_size, largest);
  }
}

pub fn build_max_heap (nums: &mut Vec<i32>, heap_size: usize) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];
  if nums.is_empty() {
    return result;
  }

  let the_last_notleaf_child: usize = parent(heap_size - 1);
  println!("the_last_notleaf_child: {}", the_last_notleaf_child);
  for i in 0..=the_last_notleaf_child {
    heapify(nums, heap_size, the_last_notleaf_child - i);
  }
  
  result
}

struct Solution ();

impl Solution {
  pub fn find_kth_largest (nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    build_max_heap(&mut nums, len);
    for i in 1..k {
      let heap_size: usize = len - i as usize;
      switch(&mut nums, 0, heap_size);  
      heapify(&mut nums, heap_size, 0);
    }

    nums[0]
  }
}


#[cfg(test)]
pub mod heap_tests {
  use super::*;

  #[test]
  fn test_find_kth_largest () {
    assert_eq!(10, Solution::find_kth_largest(vec![4, 1, 7, 2, 10, 20, 0, 2], 2));
    assert_eq!(5, Solution::find_kth_largest(vec![3,2,1,5,6,4], 2));
    assert_eq!(4, Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
    assert_eq!(3, Solution::find_kth_largest(vec![3], 1));
  }

}
