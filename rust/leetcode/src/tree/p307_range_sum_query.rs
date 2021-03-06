#[derive(Debug, Clone)]
pub(crate) struct NumArray {
  tree: Vec<i32>
}

pub(crate) fn left_child (i: usize) -> usize {
  i << 1
}

pub(crate) fn right_child (i: usize) -> usize {
  (i << 1) | 0b1
}

pub(crate) fn parent (i: usize) -> usize {
  i >> 1 as usize
}


impl NumArray {
  fn new (nums: Vec<i32>) -> Self {
    let len = nums.len();
    let mut tree_arr = vec![0; len];
    let mut tree_arr = [tree_arr, nums].concat();

    // (1..5).rev() => 4 3 2 1
    for i in (1..len).rev() {
      // now i from len-1 to 0 => [len-1, 0)
      dbg!(i);
      tree_arr[i] = tree_arr[left_child(i)] + tree_arr[right_child(i)];
    }
    
    NumArray {
      tree: tree_arr
    }
  }

  fn update (&mut self, i: usize, val: i32) {
    let n = self.tree.len() / 2;
    // i must between [0, n)
    if i >= n {
      panic!(format!("{} is out of bounds", i));
    }

    let index = n + i;
    self.tree[index] = val;
    // update parent
    let mut p = parent(index);
    // because self.tree's index is from 1
    while p != 0 {
      self.tree[p] = self.tree[left_child(p)] + self.tree[right_child(p)];
      p = parent(p);
    }
  }

  // 计算和 [i, j), 左开右闭区间
  fn sum_range (&self, i: usize, j: usize) -> i32 {
    let mut res = 0;
    let n = self.tree.len() / 2;
    let mut left = n + i;
    let mut right = n + j;

    while left < right {
      if left & 0b1 == 1 {
        res += self.tree[left];
        left += 1;
      }

      if right & 0b1 == 1 {
        right -= 1;
        res += self.tree[right];
      }

      left >>= 1;
      right >>= 1;
    }

    res
  }
}


#[cfg(test)]
mod range_sum_query_tests {
  use super::*;

  #[test]
  fn test_num_array () {
    let arr = NumArray::new(vec![1, 2, 3, 4, 5]);

    let ref_arr = &arr;
    dbg!(ref_arr);

    let sum = arr.sum_range(0, 4);
    dbg!(sum);

    assert_eq!(10, sum);

    let mut arr = arr;
    arr.update(0, 2);
    let sum = arr.sum_range(0, 4);
    assert_eq!(11, sum);
  }
}
