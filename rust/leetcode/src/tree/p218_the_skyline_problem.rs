

#[path = "../solution.rs"]
mod solution;

use solution::Solution;

#[derive(Debug)]
struct MaxSegmentTree {
  tree: Vec<(i32, i32, i32)>
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

type Dimensions = (i32, i32, i32);

impl MaxSegmentTree {

  fn new (building_dimensions: Vec<Vec<i32>>) -> Self {
    let len = building_dimensions.len();
    let normalized_building_dimensions: Vec<Dimensions> = building_dimensions.iter().map(|item| {
      (item[0], item[1], item[2])
    }).collect();

    let mut tree_arr: Vec<Dimensions> = [vec![(0,0,0); len], normalized_building_dimensions].concat();

    // build segment tree, max segment tree
    let mut index = len - 1;
    while index > 0 {
      let left_child_index = left_child(index);
      let right_child_index = right_child(index);

      let left_child_height = tree_arr[left_child_index].2;
      let right_child_height = tree_arr[right_child_index].2;
      
      tree_arr[index] = match left_child_height >= right_child_height {
        true => tree_arr[left_child_index],
        false => tree_arr[right_child_index]
      };
      
      index -= 1;
    }
    
    dbg!(&tree_arr);
    MaxSegmentTree {
      tree: tree_arr
    }
  }

  fn query (left: i32, right: i32) -> Option<Vec<(i32, i32)>> {
    if left <= right {
      return None;
    }
    
    None
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
    let st = MaxSegmentTree::new(building_dimens);
    dbg!(&st);
  }
}
