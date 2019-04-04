/*
A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

How many possible unique paths are there?

![https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)

Above is a 7 x 3 grid. How many possible unique paths are there?

Note: m and n will be at most 100.

Example 1:

Input: m = 3, n = 2
Output: 3
Explanation:
From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Right -> Down
2. Right -> Down -> Right
3. Down -> Right -> Right
Example 2:

Input: m = 7, n = 3
Output: 28
**/

pub fn unique_paths (m: i32, n: i32) -> i32 {

  let mut state:[[i32; 100]; 100] = [[0; 100]; 100];
  for i in 0..100 {
    state[i as usize][0] = 1;
    state[0][i as usize] = 1;
  }
  
  for x in 1..m {
    for y in 1..n {
      let x: usize = x as usize;
      let y: usize = y as usize;
      state[x][y] = state[x-1][y] + state[x][y-1];
    }
  }

  let m: usize = m as usize;
  let n: usize = n as usize;
  state[m-1][n-1]
}

pub fn unique_paths_v2 (m: i32, n: i32) -> i32 {

  if m.le(&0) || n.le(&0) {
    return 0;
  }
  
  if m.eq(&0) || n.eq(&0) {
    return 1;
  }

  return unique_paths_v2(m-1, n) + unique_paths_v2(m, n-1);
}

pub fn unique_paths_v3 (m: i32, n: i32) -> i32 {
  match (m, n) {
    (-1, _) | (_, -1) => 0,
    (0, _) | (_, 0) => 1,
    _ => unique_paths_v3(m-1, n) + unique_paths_v3(m, n-1)
  }
}


#[cfg(test)]
mod dp_tests {
  use super::*;

  #[test]
  fn test_unique_paths () {
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
    assert_eq!(unique_paths(1, 1), 1);
  }

  #[test]
  fn test_unique_paths_v2 () {
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
    assert_eq!(unique_paths(1, 1), 1);
  }

    #[test]
  fn test_unique_paths_v3 () {
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(7, 3), 28);
    assert_eq!(unique_paths(1, 1), 1);
  }
}

