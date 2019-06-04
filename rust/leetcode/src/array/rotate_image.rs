/// # 48. Rotate Image
/// href: https://leetcode.com/problems/rotate-image/

use std::mem;

struct Solution ();
impl Solution {
  pub fn reverse_up_to_down (matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    for row in 0..(rows / 2) {
      let another_row = rows - row - 1;
      let (left_rows, right_rows) = matrix.split_at_mut(row + 1);
      mem::swap(&mut left_rows[row], &mut right_rows[another_row - row - 1]);
    }
  }

  // first reverse up to down
  // second swap (x, y) to (y, x)
  pub fn rotate (matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    Solution::reverse_up_to_down(matrix);
    for x in 0..n {
      for y in (x + 1)..n {
        let tmp = *(&matrix[x][y]);
        matrix[x][y] = matrix[y][x];
        matrix[y][x] = tmp;
      }
    }
  }
}

#[cfg(test)]
mod array_tests {
  use super::*;

  #[test]
  pub fn test_rotate () {
    let mut matrix = vec!(
      vec!(1, 2, 3),
      vec!(4, 5, 6),
      vec!(7, 8, 9)
    );

    let expected = vec!(
      vec!(7, 4, 1),
      vec!(8, 5, 2),
      vec!(9, 6, 3)
    );
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, expected);
  }
}
