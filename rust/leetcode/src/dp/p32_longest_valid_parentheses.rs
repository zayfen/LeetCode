//! https://leetcode-cn.com/problems/longest-valid-parentheses/

//! Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

//!   Example 1:

//! Input: "(()"
//!   Output: 2
//!   Explanation: The longest valid parentheses substring is "()"
//!   Example 2:

//! Input: ")()())"
//!   Output: 4
//!   Explanation: The longest valid parentheses substring is "()()"

struct Solution ();

impl Solution {
  pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut dp: Vec<usize> = vec![0; s.chars().count()];
    let mut max: usize = 0;

    for i in 1..s.chars().count() {
      if s.get(i..i+1) == Some(")") {
        if s.get(i-1..i) == Some("(") {
          // dp[i] = dp[i-2] + 2
          dp[i] = match i {
            idx if idx < 2 => 2,
            _ => dp[i-2] + 2
          }

        } else {
          // if dp[i-dp[i-1]-1] == "(", then dp[i] = dp[i-dp[i-1]-2] + dp[i-1] + 2
          if s.get((i-dp[i-1]-1)..(i-dp[i-1])) == Some("(") {
            dp[i] = match i {
              idx if idx > dp[i-1] + 2 => dp[i-dp[i-1]-2] + dp[i-1] + 2,
              _ => dp[i-1] + 2
            }
          }
        }
        max = dp[i].max(max);
      }
    }
    max as i32
  }

  
  // 基于stack的解法，先记录每一对括号的位置，然后通过编列这些记录着多最长的合法括号
  pub fn longest_valid_parentheses_v2 (s: String) -> i32 {
    #[derive(Clone, Copy, Debug)]
    struct ParenthesesRecord (usize, usize, bool); // (leftParenthesePosition, rightParenthesePosition, visited)

    let mut max: usize = 0;
    let mut stack: Vec<usize> = vec![];
    let mut records: Vec<ParenthesesRecord> = vec![ParenthesesRecord(0,0, false); s.chars().count()];

    for (i, c) in s.chars().enumerate() {
      if c == ')' {
        if !stack.is_empty() {
          let lparenthese_index = stack.pop().unwrap();
          records[lparenthese_index] = ParenthesesRecord(lparenthese_index, i, false);
        }
      } else {
        stack.push(i);
      }
    }

    let clone_records = records.clone();
    let ref_records = &mut records;
    dbg!(&ref_records);

    fn travel (record: &mut ParenthesesRecord, records: &Vec<ParenthesesRecord>) -> usize {
      record.2 = true;
      if record.0 >= record.1 {
        return 0;
      }
      if record.1 + 1 == records.len() {
        return record.1 - record.0 + 1;
      }

      let mut next_record = records[record.1 + 1];
      return record.1 - record.0 + 1 + travel(&mut next_record, &records);
    }

    
    for record in ref_records {
      if record.2 { // this record visited
        continue;
      }

      max = max.max(travel(record, &clone_records));
    }

    max as i32
  }
}

#[cfg(test)]
mod dp_tests {
  use super::*;

  #[test]
  fn test_longest_valid_parentheses () {
    assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses(")".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses("(".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
    assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
  }


  #[test]
  fn test_longest_valid_parentheses_v2 () {
    assert_eq!(Solution::longest_valid_parentheses_v2("".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses_v2(")".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses_v2("(".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses_v2(")()())".to_owned()), 4);
    assert_eq!(Solution::longest_valid_parentheses_v2("(()".to_owned()), 2);
    assert_eq!(Solution::longest_valid_parentheses_v2("()(()".to_owned()), 2);
  }
}

