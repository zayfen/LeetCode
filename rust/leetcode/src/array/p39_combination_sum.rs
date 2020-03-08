/// p39_combination_sum
struct Solution();

impl Solution {

  pub fn combination_sum (candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = [].to_vec();
    let mut candidates = candidates;
    candidates.sort_unstable();
    let len = candidates.len();
    if len == 0 {
      return result;
    }

    let mut path: Vec<i32> = [].to_vec();
    Solution::combination_sum_helper(&candidates, target, 0, &mut path, &mut result);
    return result;
  }


  pub fn combination_sum_helper (candidates: &Vec<i32>, target: i32, begin: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {

    if target == 0 {
      result.push(path.clone());
      return;
    }
    let len = candidates.len();
    let mut index = begin;
    
    while index < len {
      let value = *(candidates.get(index).unwrap_or(&0));
      if value <= target {
        path.push(value);
        Solution::combination_sum_helper(candidates, target - value, index, path, result);
        path.pop();
      }

      index += 1;
    }
  }
}

#[cfg(test)]
mod test_array {
  use super::*;

  #[test]
  fn test_combination_sum () {
    let res = Solution::combination_sum(vec![2,3,6,7], 7);
    println!("{:?}", res);
    assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
  }
}

