/*

A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

Now consider if some obstacles are added to the grids. How many unique paths would there be?



An obstacle and empty space is marked as 1 and 0 respectively in the grid.

Note: m and n will be at most 100.

Example 1:

Input:
[
  [0,0,0],
  [0,1,0],
  [0,0,0]
]
Output: 2
Explanation:
There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right

 */

pub fn unique_paths_with_obstacles (obstacle_grid: Vec<Vec<i32>>) -> i32 {
  let mut state:[[i32; 100]; 100] = [[-1; 100]; 100];

  if let 1 = obstacle_grid[0][0] {
    state[0][0] = 0;
  } else {
    state[0][0] = 1;
  }

  let (len_rows, len_cols) = (obstacle_grid.len(), obstacle_grid[0].len());
  for row in 0..len_rows {
    for col in 0..len_cols {
      match obstacle_grid[row][col] {
        1 => {                  // hava obstacle
          state[row][col] = 0;
        },
        _ => { // no obstacle
          match (row as i32, col as i32) {
            (0, 0) => continue,
            (0, _) => state[row][col] = state[row][col-1],
            (_, 0) => state[row][col] = state[row-1][col],
            (_, _) => state[row][col] = state[row-1][col] + state[row][col-1]
          }
        }
      }                         
    }
  }

  state[len_rows-1][len_cols-1]
}

pub fn unique_patghs_with_obstacles_v2 (obstacle_grid: Vec<Vec<i32>>) -> i32 {
  0
}

#[cfg(test)]
mod dp_tests {
  use super::*;

  #[test]
  fn test_unique_paths_with_obstacles () {
    let vec2d = vec!(vec!(0, 0, 0), vec!(0, 1, 0), vec!(0, 0, 0));
    assert_eq!(unique_paths_with_obstacles(vec2d), 2);
  }

  #[test]
  fn test_unique_paths_with_obstacles_v2 () {
    let vec2d = vec!(vec!(0, 0, 0), vec!(0, 1, 0), vec!(0, 0, 0));
    assert_eq!(unique_paths_with_obstacles(vec2d), 2);
  }
}
