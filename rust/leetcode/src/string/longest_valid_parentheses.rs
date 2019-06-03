/// # 32. Longest Valid Parentheses
/// > href: https://leetcode.com/problems/longest-valid-parentheses/


struct Solution ();
impl Solution {
  pub fn longest_valid_parentheses (s: String) -> i32 {
    let mut stack = vec!();
    let mut chars_iter = s.char_indices();

    let mut longest = 0;
    let mut longest_recorder = vec!();

    while let Some((index, ch)) = chars_iter.next() {
      if ch == '(' {
        stack.push((index, ch));
        continue;
      }

      // ch is ')', pop top '('
      if stack.len() > 0 {
        let paired_parent = stack.pop();
        longest_recorder.push(paired_parent.unwrap());
        continue;
      }

      // stack is empty,
      if longest_recorder.len() as i32 > longest {
        longest = longest_recorder.len() as i32;
      }

      longest_recorder.clear();
    }


    let mut stack_top = stack.pop();
    let mut counter = 0;
    while longest_recorder.len() > 0 {
      if stack_top == None {
        counter = longest_recorder.len() as i32;
        longest = if counter > longest { counter } else { longest };
        break;
      }

      let recorder = longest_recorder.pop().unwrap();

      if stack_top.unwrap().0 < recorder.0 {
        counter += 1;

        longest = if counter > longest { counter } else { longest };
        continue;
      }

      longest_recorder.push(recorder);
      counter = 0;
      stack_top = stack.pop();
    }

    longest * 2
  }
}


#[cfg(test)]
mod string_tests {
  use super::*;

  #[test]
  pub fn test_longest_valid_parentheses () {
    let s1 = String::from("(()");
    assert_eq!(Solution::longest_valid_parentheses(s1), 2);

    let s2 = String::from(")()())");
    assert_eq!(Solution::longest_valid_parentheses(s2), 4);

    let s3 = String::from("()(()");
    assert_eq!(Solution::longest_valid_parentheses(s3), 2);

    let s4 = String::from("()(()(()");
    assert_eq!(Solution::longest_valid_parentheses(s4), 2);
  }
}
