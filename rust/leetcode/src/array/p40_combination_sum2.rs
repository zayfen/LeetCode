struct Solution();

impl Solution {
  pub fn combination_sum2 (candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    if candidates.len() == 0 {
      return vec![];
    }
    candidates.sort_unstable();
    let mut result: Vec<Vec<i32>> = vec![];
    let mut path: Vec<i32> = vec![];
    
    Solution::combination_sum2_helper(&candidates, target, 0, &mut path, &mut result);

    return result;
  }

  pub fn combination_sum2_helper (candidates: &Vec<i32>, target: i32, begin: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if target == 0 {
      result.push(path.clone());
      return;
    }

    let mut index = begin;
    let len = candidates.len();
    
    while index < len {
      let value = candidates[index];

      if value > target {
        index += 1;
        continue;
      }
      // 如果当前数字和前一个数字相同,则跳过
      if index > begin && candidates[index] == candidates[index-1] {
        index += 1;
        continue;
      }

      path.push(value);
      Solution::combination_sum2_helper(candidates, target - value, index + 1, path, result);
      path.pop();
      
      index += 1;
    }
  }
}


#[cfg(test)]
mod test_array {
  use super::*;

  #[test]
  fn test_combination_sum2 () {
    let candidates = vec![10,1,2,7,6,1,5];
    // sorted: 1 1 2 5 6 7 10
    let target = 8;
    let result = Solution::combination_sum2(candidates, target);
    assert_eq!(result, vec![ vec![1,1,6], vec![1,2,5], vec![1,7], vec![2,6] ]);
  }
}
