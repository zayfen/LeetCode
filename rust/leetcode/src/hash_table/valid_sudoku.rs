/// https://leetcode.com/problems/valid-sudoku/


struct Solution ();

impl Solution {

  pub fn is_valid_cell_in_row (board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut numbers = vec!('1', '2', '3', '4', '5', '6', '7', '8', '9');

    for num in &board[row] {
      if *num == '.' {
        continue;
      }

      let number = num.to_digit(10).unwrap();
      if numbers[(number-1) as usize] == '.' {
        return false;
      } else {
        numbers[(number-1) as usize] = '.';
      }
    }

    return true;
  }

  pub fn is_valid_cell_in_column (board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut numbers = vec!('1', '2', '3', '4', '5', '6', '7', '8', '9');
    for r in 0..9 {
      let n = board[r][col];
      if n == '.' {
        continue;
      }
      let number = n.to_digit(10).unwrap();
      if numbers[(number - 1) as usize] == '.' {
        return false;
      } else {
        numbers[(number - 1) as usize] = '.'
      }
    }
    return true;
  }

  pub fn is_valid_cell_in_block (board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut numbers = vec!('1', '2', '3', '4', '5', '6', '7', '8', '9');
    let x: usize = (row as f32 / 3f32).floor() as usize;
    let y: usize = (col as f32 / 3f32).floor() as usize;
    let (cell_index_x, cell_index_y) = (x * 3, y * 3);
    for i in 0..3 {
      for j in 0..3 {
        if board[cell_index_x + i][cell_index_y + j] == '.' {
          continue;
        }
        let number = board[cell_index_x + i][cell_index_y + j].to_digit(10).unwrap();
        if numbers[(number-1) as usize] == '.' {
          return false;
        } else {
          numbers[(number-1) as usize] = '.';
        }
      }
    }
    return true;
  }

  pub fn is_valid_sudoku (board: Vec<Vec<char>>) -> bool {
    for row in 0..9 {
      for col in 0..9 {
        if board[row][col] != '.' {
          let pass = Solution::is_valid_cell_in_row(&board, row, col) && Solution::is_valid_cell_in_column(&board, row, col) && Solution::is_valid_cell_in_block(&board, row, col);
          if  !pass {
            return false;
          }
        }
      }
    }

    return true;
  }

  pub fn is_valid_sudoku_v2 (board: Vec<Vec<char>>) -> bool {
    let mut hash_map = std::collections::HashMap::new();

    for row in 0..9 {
      for col in 0..9 {
        let ch = board[row][col];
        if ch != '.' {
          let x: usize = (row as f32 / 3f32).floor() as usize;
          let y: usize = (col as f32 / 3f32).floor() as usize;
          let (cell_index_x, cell_index_y) = (x * 3, y * 3);

          let row_key = format!("{}r{}", ch, row);
          let col_key = format!("{}c{}", ch, col);
          let block_key = format!("{}b{:?}", ch, (cell_index_x, cell_index_y));
          if hash_map.insert(row_key, "") == None &&
            hash_map.insert(col_key, "") == None &&
            hash_map.insert(block_key, "") == None {
              continue;
            }
          return false;
        }
      }
    }

    return true;
  }

}

#[cfg(test)]
mod hash_table {
  use super::*;

  #[test]
  fn test_is_valid_sudoku () {
    let board = vec!(
      vec!('5','3','.','.','7','.','.','.','.'),
      vec!('6','.','.','1','9','5','.','.','.'),
      vec!('.','9','8','.','.','.','.','6','.'),
      vec!('8','.','.','.','6','.','.','.','3'),
      vec!('4','.','.','8','.','3','.','.','1'),
      vec!('7','.','.','.','2','.','.','.','6'),
      vec!('.','6','.','.','.','.','2','8','.'),
      vec!('.','.','.','4','1','9','.','.','5'),
      vec!('.','.','.','.','8','.','.','7','9')
    );

    assert_eq!(Solution::is_valid_sudoku_v2(board), true);
  }
}
