/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 *
 * https://leetcode-cn.com/problems/wildcard-matching/description/
 *
 * algorithms
 * Hard (27.29%)
 * Likes:    306
 * Dislikes: 0
 * Total Accepted:    25.2K
 * Total Submissions: 92.3K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给定一个字符串 (s) 和一个字符模式 (p) ，实现一个支持 '?' 和 '*' 的通配符匹配。
 * 
 * '?' 可以匹配任何单个字符。
 * '*' 可以匹配任意字符串（包括空字符串）。
 * 
 * 
 * 两个字符串完全匹配才算匹配成功。
 * c:\Users\zayfen\.vscode\extensions\shengchen.vscode-leetcode-0.16.1\node_modules
 * 说明:
 * 
 * 
 * s 可能为空，且只包含从 a-z 的小写字母。
 * p 可能为空，且只包含从 a-z 的小写字母，以及字符 ? 和 *。
 * 
 * 
 * 示例 1:
 * 
 * 输入:
 * s = "aa"
 * p = "a"
 * 输出: false
 * 解释: "a" 无法匹配 "aa" 整个字符串。
 * 
 * 示例 2:
 * 
 * 输入:
 * s = "aa"
 * p = "*"
 * 输出: true
 * 解释: '*' 可以匹配任意字符串。
 * 
 * 
 * 示例 3:
 * 
 * 输入:
 * s = "cb"
 * p = "?a"
 * 输出: false
 * 解释: '?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。
 * 
 * 
 * 示例 4:
 * 
 * 输入:
 * s = "adceb"
 * p = "*a*b"
 * 输出: true
 * 解释: 第一个 '*' 可以匹配空字符串, 第二个 '*' 可以匹配字符串 "dce".
 * 
 * 
 * 示例 5:
 * 
 * 输入:
 * s = "acdcb"
 * p = "a*c?b"
 * 输入: false
 * 
 */

// @lc code=start
struct Solution();

fn dfs (dp: &mut Vec<usize>, pindex: usize, s: &String, p: &String) -> bool {
  if p.len() == 0 {
    return s.len() == 0;
  }

  if pindex >= p.len() {
    return dp[p.len()-1] == s.len();
  }
  
  let mut matched = false;
  let current_pattern_char = p.chars().nth(pindex).unwrap();
  match current_pattern_char {
    '*' => { // 因为 ‘×’ 可以匹配空字符串
      if s.is_empty() {
        return p.len() == 1;
      }

      let mut start_index = 0;
      if pindex > 0 {
        start_index = dp[pindex - 1];
      }
      let end = s.len() + 1;
      
      for i in start_index..end {
        if matched { // 如果已经匹配上了，就break;
          break;
        }
        // p[pindex+1] == s[dp[pindex]]
        dp[pindex] = i;
        if pindex + 1 < (p.len() - 1) && i < s.len() {
          let next_pattern_char = p.chars().nth(pindex+1).unwrap();
          // find next_pattern_char 对应在s中的位置
          if s.chars().nth(i).unwrap() != next_pattern_char && next_pattern_char != '?' {
            continue;
          }
        }

        matched = dfs(dp, pindex + 1, s, p);
      }
    },
    c => {
      let mut source_index = 0;
      if pindex > 0 {
        source_index = dp[pindex - 1];
      }
      if source_index >= s.len() { // dp[index-1]已经匹配了所有的字符串
        return false;
      }
      if c == s.chars().nth(source_index).unwrap() || c == '?' {         // ? 匹配任何字符串
        dp[pindex] = source_index + 1;
        return dfs(dp, pindex + 1, s, p);
      } else {
        return false;
      }
    }
  }
  
  matched
}

fn simplify_pattern (p: String) -> String {
  let mut result: String = "".to_owned();
  let mut p = p;
  while p != result {
    result = p.clone();
    p = p.replace("**", "*");
  }
  result
}

impl Solution {


  pub fn is_match(s: String, p: String) -> bool {
    // p 替换连续的*成一个
    let p = simplify_pattern(p);
    let mut dp: Vec<usize> = vec![0; p.len()];
    return dfs(&mut dp, 0, &s, &p);
  }
}
// @lc code=end


#[cfg(test)]
mod dp_tests {
  use super::*;

  #[test]
  fn test_is_match () {

    assert_eq!(true, Solution::is_match("".to_owned(), "".to_owned()));
    assert_eq!(true, Solution::is_match("".to_owned(), "*".to_owned()));
    assert_eq!(false, Solution::is_match("".to_owned(), "?".to_owned()));
    assert_eq!(false, Solution::is_match("".to_owned(), "*a".to_owned()));
    assert_eq!(false, Solution::is_match("".to_owned(), "?a".to_owned()));
    assert_eq!(false, Solution::is_match("a".to_owned(), "?a".to_owned()));
    assert_eq!(true, Solution::is_match("a".to_owned(), "*a".to_owned()));
    assert_eq!(false, Solution::is_match("aa".to_owned(), "a".to_owned()));
    assert_eq!(true, Solution::is_match("aa".to_owned(), "a*".to_owned()));
    assert_eq!(true, Solution::is_match("aa".to_owned(), "aa".to_owned()));
    assert_eq!(true, Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
    assert_eq!(false, Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()));
    assert_eq!(false, Solution::is_match("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_owned(), "a*******b".to_owned()));
    assert_eq!(false, Solution::is_match("babbbbaabababaabbababaababaabbaabababbaaababbababaaaaaabbabaaaabababbabbababbbaaaababbbabbbbbbbbbbaabbb".to_owned(), "b**bb**a**bba*b**a*bbb**aba***babbb*aa****aabb*bbb***a".to_owned()));
  }
}
