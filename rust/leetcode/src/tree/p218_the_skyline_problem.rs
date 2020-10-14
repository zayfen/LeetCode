#[path = "../solution.rs"]
mod solution;

use solution::Solution;

type TreeNode = (i32, i32);

#[derive(Debug)]
struct MaxSegmentTree {
  buildings: Vec<Vec<i32>>,
  st_buildings: Option<Vec<TreeNode>>
}

pub(crate) fn left_child (i: usize) -> usize {
  i << 1
}

pub(crate) fn right_child (i: usize) -> usize {
  (i << 1) | 0b1
}

pub(crate) fn parent (i: usize) -> usize {
  i >> 1
}

impl MaxSegmentTree {

  fn new (buildings: Vec<Vec<i32>>) -> Self {
    MaxSegmentTree {
      buildings,
      st_buildings: None
    }
  }

  /// # 查询区间
  /// @param {{ i32 }} left 左边界 闭区间
  /// @param {{ i32 }} right 右边界 开区间
  /// @returns {{ Option<Vec<(i32, i32)>> }}
  fn query (left: i32, right: i32) -> Option<Vec<(i32, i32)>> {
    if left <= right {
      return None;
    }
    
    None
  }

  fn build (&mut self) {
    let mut flattened_buildings: Vec<TreeNode> = self.buildings.iter().map(|item| {
      vec![(item[0], item[2]), (item[1], item[2])]
    }).flatten().collect::<Vec<TreeNode>>();

    let len = flattened_buildings.len();
    let mut st_tree: Vec<TreeNode> = [vec![(0, 0); len], flattened_buildings].concat();

    let mut index = len - 1;
    while index > 0 {
      let left_child_index = left_child(index);
      let right_child_index = right_child(index);

      let left_child_height = st_tree[left_child_index].1;
      let right_child_height = st_tree[right_child_index].1;

      st_tree[index] = match left_child_height >= right_child_height {
        true => st_tree[left_child_index],
        false => st_tree[right_child_index]
      };

      index -= 1;
    }
    dbg!(st_tree.clone());
    self.st_buildings = Some(st_tree);
  }
}


impl Solution {
  
  pub fn get_skyline (building: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]];
    
    res
  }
}

#[cfg(test)]
mod get_skyline_test {
  use super::*;

  #[test]
  fn test_get_skyline () {
    let building_dimens = vec![ vec![2,9,10], vec![3,7,15], vec![5,12,12], vec![15,20,10], vec![19,24,8]];
    let mut st = MaxSegmentTree::new(building_dimens);
    st.build();
    dbg!(&st);
  }
}
