//
pub fn max_sub_array (nums: Vec<i32>) -> i32 {
  // f(n) = max(f(n-1), f(n-1) + nums[n])

  // init
  let mut memo = Vec::new();
  memo.push(*nums.get(0).unwrap());

  let mut recorder: Vec<i32> = Vec::new();
  recorder.push(*nums.get(0).unwrap());

  let mut result: Vec<i32> = Vec::new();

  let len_nums = nums.len();
  for num in 1..len_nums {
    let prev_num = *memo.get(num-1).unwrap();
    let current_num = *nums.get(num).unwrap();
    memo.push(prev_num.max(prev_num + current_num));

    if prev_num + current_num >= prev_num { // select current number, save sum
      recorder.push(current_num);
    } else {
      // save sum of this sub_array
      println!("{:?}", recorder);
      result.push(recorder.iter().sum());
      recorder.clear();
    }
  }

  println!("{:?}", result);

  *result.iter().max().unwrap()
}

#[cfg(test)]
mod dp_tests {
  use super::*;

  #[test]
  fn test_max_sub_array () {
    assert_eq!(6, max_sub_array(vec!(-2, 1, -3, 4, -1, 2, 1, -5, 4)));
  }
}
