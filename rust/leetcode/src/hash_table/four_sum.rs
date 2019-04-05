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
  
  if n == 0 {                   // print result
    if target == 0 {
      println!("{:?}", result);

      if store.len() > 0 {
        let last_result = &store[store.len() - 1];
        if !(last_result.iter().zip(&result).all(|(a, b)| *a == *b)) {
          store.push(result);
        }

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
    if target >= number {
      let next_nums = &nums[i+1..];
      let next_target = target - number;
      let mut next_result = result.clone();

      next_result.push(number);
      n_sum(next_nums, next_target, n-1, next_result, store);
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
