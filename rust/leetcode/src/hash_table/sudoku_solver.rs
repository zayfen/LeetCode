/// leetcode.com/problems/sudoku-solver/

struct Solution ();


impl Solution {
  pub fn valid_numbers (board: &Vec<Vec<char>>, row: usize, column: usize) -> Vec<char> {
    let mut numbers = vec!('1', '2', '3', '4', '5', '6', '7', '8', '9');
    
    // mark row
    for num in &board[row] {
      if *num == '.' {
        continue;
      }
      
      let number = num.to_digit(10).unwrap();
      numbers[(number-1) as usize] = '.';
    }
    
    // mark column
    for r in 0..9 {
      let n = board[r][column];
      if n == '.' {
        continue;
      }
      let n = n.to_digit(10).unwrap();
      numbers[(n-1) as usize] = '.'
    }

    // 3 x 3 round
    let x: usize = (row as f32 / 3f32).floor() as usize;
    let y: usize = (column as f32 / 3f32).floor() as usize;
    let (cell_index_x, cell_index_y) = (x * 3, y * 3);
    for i in 0..3 {
      for j in 0..3 {
        if board[x + i][y + j] == '.' {
          continue;
        }
        let number = board[x + i][y + j].to_digit(10).unwrap();
        numbers[(number-1) as usize] = '.';
      }
    }

    numbers.retain(|&c| c != '.');
    numbers
  }

  pub fn solve_sudoku_helper (board: &mut Vec<Vec<char>>, (row: usize, col: usize), prev_valid_chars: &mut Vec<char>) {
    let (mut next_row, mut next_col) = (row, col + 1);
    if next_col >= 9 {
      next_col = 0;
      next_row += 1;
    }

    if (next_row >= 9) { // reach the end
      Solution::print_sudoku(board);
    }


    let (mut prev_row, mut prev_col) = (row, col - 1);
    if prev_col < 0 {
      prev_col = 8;
      prev_row -= 1; // NOTE: if prev_row < 0, so that current position is (0, 0)
    }
    
    if board[row][col] == '.' {
      let valid_chars = Solution::valid_numbers(board, row, col);
      let no_valid_chars = valid_chars.len() == 0;
      if no_valid_chars && prev_row < 0 {
        return -10000; // Bad Sudoku
      }
      if no_valid_chars { // 回溯
        prev_valid_chars.remove(0);
        board[prev_row][prev_col] = '.';
        Solution::solve_sudoku_helper(board, prev_row, prev_col, prev_valid_chars);

      } else {
        
        for c in valid_chars {
          board[row][col] = c;
          Solution::solve_sudoku_helper(board, next_row, next_col);
        }
      }
      
      
    } else {
      Solution::solve_sudoku_helper(board, next_row, next_col);
    }
  }

  

  pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let (row, col) = (100, 100);
    // find the first dot (empty cell)
    for x in 0..9 {
      for y in 0..9 {
        if board[x][y] == '.' {
          row = x;
          col = y;
          break;
        }
      }
    }

    if row == 100 { // sudoku is finished already
      Solution::print_sudoku(board);
      return;
    }

    Solution::solve_sudoku_helper(board, row, col);
  }
  

  pub fn print_sudoku(board: &Vec<Vec<char>>) {
    println!("=============================\n");
    println!("Solved Sudoku:\n {:?} \n", board);
    println!("=============================\n");
  }
}

#[cfg(test)]
mod hash_table {
  use super::*;

  #[test]
  fn test_solve_sudoku () {
    let mut board = vec!(
      vec!('5', '3', '.', '.', '7', '.', '.', '.', '.'),
      vec!('6', '.', '.', '1', '9', '5', '.', '.', '.'),
      vec!('.', '9', '8', '.', '.', '.', '.', '6', '.'),
      vec!('8', '.', '.', '.', '6', '.', '.', '.', '3'),
      vec!('4', '.', '.', '8', '.', '3', '.', '.', '1'),
      vec!('7', '.', '.', '.', '2', '.', '.', '.', '6'),
      vec!('.', '6', '.', '.', '.', '.', '2', '8', '.'),
      vec!('.', '.', '.', '4', '1', '9', '.', '.', '5'),
      vec!('.', '.', '.', '.', '8', '.', '.', '7', '9')
    );

    Solution::solve_sudoku(& mut board);

    let expected_board = vec!(
      vec!('5', '3', '4', '6', '7', '8', '9', '1', '2'),
      vec!('6', '7', '2', '1', '9', '5', '3', '4', '8'),
      vec!('1', '9', '8', '3', '4', '2', '5', '6', '7'),
      vec!('8', '5', '9', '7', '6', '1', '4', '2', '3'),
      vec!('4', '2', '6', '8', '5', '3', '7', '9', '1'),
      vec!('7', '1', '3', '9', '2', '4', '8', '5', '6'),
      vec!('9', '6', '1', '5', '3', '7', '2', '8', '4'),
      vec!('2', '8', '7', '4', '1', '9', '6', '3', '5'),
      vec!('3', '4', '5', '2', '8', '6', '1', '7', '9')
    );
    assert_eq!(board, expected_board);
  }

  #[test]
  fn test_valid_number () {
    let mut board = vec!(
      vec!('5', '3', '.', '.', '7', '.', '.', '.', '.'),
      vec!('6', '.', '.', '1', '9', '5', '.', '.', '.'),
      vec!('.', '9', '8', '.', '.', '.', '.', '6', '.'),
      vec!('8', '.', '.', '.', '6', '.', '.', '.', '3'),
      vec!('4', '.', '.', '8', '.', '3', '.', '.', '1'),
      vec!('7', '.', '.', '.', '2', '.', '.', '.', '6'),
      vec!('.', '6', '.', '.', '.', '.', '2', '8', '.'),
      vec!('.', '.', '.', '4', '1', '9', '.', '.', '5'),
      vec!('.', '.', '.', '.', '8', '.', '.', '7', '9')
    );

    assert_eq!(Solution::valid_numbers(&board, 0, 2), vec!('1', '2', '3'));
  }
}
