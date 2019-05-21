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
        if board[cell_index_x + i][cell_index_y + j] == '.' {
          continue;
        }
        let number = board[cell_index_x + i][cell_index_y + j].to_digit(10).unwrap();
        numbers[(number-1) as usize] = '.';
      }
    }

    numbers.retain(|&c| c != '.');
    numbers
  }
  
  pub fn solve_sudoku_helper (board: &mut Vec<Vec<char>>, row: usize, col: usize, solution: &mut Vec<Vec<char>>) {

    let (mut next_row, mut next_col) = (row, col + 1);
    if next_col >= 9 {
      next_col = 0;
      next_row += 1;
    }

    if row > 8 || (row == 8 && col == 8 && board[row][col] != '.') { // reach the end
      Solution::print_sudoku(board);
      for r in 0..9 {
        for c in 0..9 {
          solution[r][c] = board[r][c];
        }
      }
      return
    }

    if board[row][col] == '.' {
      let valid_chars = Solution::valid_numbers(board, row, col);
      
      if row == 8 && col == 8 {
        println!("valid_chars {:?}", valid_chars);
      }
      
      let no_valid_chars = valid_chars.len() == 0;

      if no_valid_chars { // 回溯
        return;

      } else {
        
        for c in valid_chars {
          board[row][col] = c;
          Solution::solve_sudoku_helper(board, next_row, next_col, solution);
        }
        board[row][col] = '.'; // restore           
      }
      
    } else {
      Solution::solve_sudoku_helper(board, next_row, next_col, solution);
    }
  }

  

  pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let (mut row, mut col) = (100, 100);
    // find the first dot (empty cell)
    for x in 0..9 {
      for y in 0..9 {
        if board[x][y] == '.' {
          row = x;
          col = y;
          break;
        }
      }
      
      if row != 100 { // break outer for loop
        break;
      }
    }

    if row == 100 { // sudoku is finished already
      Solution::print_sudoku(board);
      return;
    }

    let mut solution = board.clone();
    Solution::solve_sudoku_helper(board, row, col, &mut solution);
    
    *board = solution;
  }
  

  pub fn print_sudoku(board: &Vec<Vec<char>>) {
    println!("=============================\n");
    for row in board {
      println!("{:?}\n", row);
    }
    println!("");
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

    let mut board2 = vec!(
      vec!('.','.','9','7','4','8','.','.','.'),
      vec!('7','.','.','.','.','.','.','.','.'),
      vec!('.','2','.','1','.','9','.','.','.'),
      vec!('.','.','7','.','.','.','2','4','.'),
      vec!('.','6','4','.','1','.','5','9','.'),
      vec!('.','9','8','.','.','.','3','.','.'),
      vec!('.','.','.','8','.','3','.','2','.'),
      vec!('.','.','.','.','.','.','.','.','6'),
      vec!('.','.','.','2','7','5','9','.','.')
    );
    
    let expected_board2 = vec!(
      vec!('5','1','9','7','4','8','6','3','2'),
      vec!('7','8','3','6','5','2','4','1','9'),
      vec!('4','2','6','1','3','9','8','7','5'),
      vec!('3','5','7','9','8','6','2','4','1'),
      vec!('2','6','4','3','1','7','5','9','8'),
      vec!('1','9','8','5','2','4','3','6','7'),
      vec!('9','7','5','8','6','3','1','2','4'),
      vec!('8','3','2','4','9','1','7','5','6'),
      vec!('6','4','1','2','7','5','9','8','3')
    );
    Solution::solve_sudoku(& mut board2);
    assert_eq!(board2, expected_board2);
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

    assert_eq!(Solution::valid_numbers(&board, 0, 2), vec!('1', '2', '4'));
  }
}
