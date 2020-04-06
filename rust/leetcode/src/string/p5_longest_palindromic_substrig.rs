struct Solution ();


impl Solution {
  pub fn longest_palindrome (s: String) -> String {
    // manacher algorithm
    let mut result: String = String::from("");

    if s.is_empty() {
      return result;
    }

    let iter = s.split("");
    let mut vec: Vec<&str> = vec![];
    vec.extend(iter);
    let s = vec.join("#");
    
    // d1[]
    let len = s.len();
    let mut mem: Vec<usize> = vec![];
    unsafe { mem.set_len(len); }
    let mut mem: Vec<usize> = mem.iter().map(|_| 1usize).collect();
    let (mut l, mut r) = (0usize, 0usize);
    
    for (index, _) in s.chars().enumerate() {
      let mut k = if index == 0 || index > r {
        1
      } else {
        mem[l + r - index].min(r - index)
      };

      while index >= k && index + k < len && s.get(index-k..index+1-k) == s.get(index+k..index+k+1) {
        k = k + 1;
      }

      mem[index] = k;  // k defiantly greater than 1
      if index + k - 1 > r {
        // update l and r
        l = index + 1 - k;
        r = index + k - 1;

        let _r: String = s.get(l..r+1).unwrap().to_owned().split("#").collect();
        if _r.len() > result.len() {
          result = _r
        }
      }
    }
    println!("{:?}", result);
    result
  }
}

#[cfg(test)]
mod test_dp {
  use super::Solution;

  #[test]
  fn test_longest_palindrome () {
    assert_eq!("bab", Solution::longest_palindrome("babad".to_owned()));
    assert_eq!("bb", Solution::longest_palindrome("cbbd".to_owned()));
    assert_eq!("a", Solution::longest_palindrome("a".to_owned()));
  }
}
