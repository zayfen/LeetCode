///
/// Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
///
/// # Note:
///
/// The solution set must not contain duplicate quadruplets.
///
/// # Example:
///
/// Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
///
/// A solution set is:
/// [
///   [-1,  0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2,  0, 0, 2]
/// ]
///
/// pub fn four_sum (nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
///   let len_nums = nums.len();
///   for i in 0..len_nums {
///     if nums[i] < target {
///
///     }
///   }
/// }

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut store: Vec<Vec<i32>> = Vec::new();
  let mut result: Vec<i32> = Vec::new();
  let mut nums = nums.clone();
  nums.sort();
  let nums  = nums.clone();

  n_sum(nums.as_slice(), target, 4, result, &mut store);
  store
}



// assume nums is sorted
pub fn n_sum (nums: &[i32], target: i32, n: i32, mut result: Vec<i32>, mut store: &mut Vec<Vec<i32>>) {
  println!("n: {:?} ; result: {:?}", n, result);
  if n == 0 {                   // print result
  
    if target == 0 {
      println!("{:?}", result);

      if store.len() > 0 {
        let mut existed = false;
        for item in store.iter() {
          if item.iter().zip(&result).all(|(a, b)| *a == *b) {
            existed = true;
          }
        }
        
        if !existed {
          store.push(result);
        }
        // let last_result = &store[store.len() - 1];
        // if !(last_result.iter().zip(&result).all(|(a, b)| *a == *b)) {
        //   store.push(result);
        // }
        
      } else {
        store.push(result);
      }

    }
    return ();
  }

  let len_nums = nums.len();
  
  for i in 0..len_nums {
    // find a number less than target
    let number = nums[i];

    // if target-number >= number, go on
    // if target >= number {
    let next_nums = &nums[i+1..];
    let next_target = target - number;
    if next_target.abs() <= target.abs() {
      let mut next_result = result.clone();
      next_result.push(number);
      n_sum(next_nums, next_target, n-1, next_result, store);
    }
  }
  
}


pub fn n_sum_v2 (nums: &[i32], target: i32, n: i32, mut result: Vec<i32>, mut store: &mut Vec<Vec<i32>>) {
  let len_nums = nums.len();
  // early return
  if len_nums < 2 || n < 2 || nums[0] * n > target || target > nums.last().unwrap() * n {
    return ()
  }
  
  if n == 2 {
    let mut left = 0;
    let mut right = len_nums - 1;
    while left < right {
      let sum = nums[left] + nums[right];
      let mut result = result.clone();
      println!("nums: {:?}", nums);
      println!("n_sum_v2 left: {:?} ;right: {:?} ;sum: {:?} ;target: {:?}", left, right, sum, target);
      if sum == target {
        result.push(nums[left]);
        result.push(nums[right]);
        store.push(result);
        left += 1;
        while left < right && nums[left] == nums[left-1] {
          left += 1;
        }
      } else if sum < target {
        left += 1;
      } else {
        right -= 1;
      }
    }
  } else {
    let i32_len_nums = len_nums as i32;
    let range_index = i32_len_nums - n;
    let mut i = 0;
    while i <= range_index {
      let i32_i = i as usize;
      if i == 0 || (i > 0 && nums[i32_i-1] != nums[i32_i]) {
        let number = nums[i32_i];
        let mut result = result.clone();
        result.push(number);
        n_sum_v2(&nums[(i32_i+1)..len_nums], target-number, n-1, result, store);
      }
      
      i+=1;
    }
  }
}


#[cfg(test)]
mod hash_table_tests {
  use super::*;
  
  #[test]
  fn test_four_sum () {
    let mut result: Vec<i32> = Vec::new();
  }
}
