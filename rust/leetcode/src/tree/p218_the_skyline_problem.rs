#[path = "../solution.rs"]
mod solution;

use solution::Solution;
use std::cmp::Ordering;

type TreeNode = (i32, i32);

// #[derive(Debug)]
// struct MaxSegmentTree {
//   buildings: Vec<Vec<i32>>,
//   st_buildings: Option<Vec<TreeNode>>,
// }

// pub(crate) fn left_child(i: usize) -> usize {
//   i << 1
// }

// pub(crate) fn right_child(i: usize) -> usize {
//   (i << 1) | 0b1
// }

// pub(crate) fn parent(i: usize) -> usize {
//   i >> 1
// }

// impl MaxSegmentTree {
//   fn new(buildings: Vec<Vec<i32>>) -> Self {
//     MaxSegmentTree {
//       buildings,
//       st_buildings: None,
//     }
//   }

//   /// # 查询区间
//   /// @param {{ i32 }} left 左边界 闭区间
//   /// @param {{ i32 }} right 右边界 开区间
//   /// @returns {{ Option<Vec<(i32, i32)>> }}
//   fn query(left: i32, right: i32) -> Option<Vec<(i32, i32)>> {
//     if left <= right {
//       return None;
//     }
//     None
//   }

//   fn build(&mut self) {
//     let mut flattened_buildings: Vec<TreeNode> = self
//       .buildings
//       .iter()
//       .map(|item| vec![(item[0], item[2]), (item[1], item[2])])
//       .flatten()
//       .collect::<Vec<TreeNode>>();

//     let len = flattened_buildings.len();
//     let mut st_tree: Vec<TreeNode> = [vec![(0, 0); len], flattened_buildings].concat();

//     let mut index = len - 1;
//     while index > 0 {
//       let left_child_index = left_child(index);
//       let right_child_index = right_child(index);

//       let left_child_height = st_tree[left_child_index].1;
//       let right_child_height = st_tree[right_child_index].1;

//       st_tree[index] = match left_child_height >= right_child_height {
//         true => st_tree[left_child_index],
//         false => st_tree[right_child_index],
//       };

//       index -= 1;
//     }
//     dbg!(st_tree.clone());
//     self.st_buildings = Some(st_tree);
//   }
// }

impl Solution {
  // 扫描法
  pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut spread_buildings: Vec<TreeNode> = buildings
      .iter()
      .map(|item| {
        vec![
          (item[0], item[2]) as TreeNode,
          (item[1], -item[2]) as TreeNode,
        ]
      }) // make right side height as minus
      .flatten()
      .collect::<Vec<TreeNode>>();

    spread_buildings.sort_by(|a: &TreeNode, b: &TreeNode| -> Ordering {
      match a.0 - b.0 {
        0 => {
          if a.1 > 0 && b.1 > 0 {
            // both are left sides
            b.1.cmp(&a.1)
          } else if a.1 < 0 && b.1 < 0 {
            // both are right sides
            b.1.cmp(&a.1)
          } else {
            // only one of them is left side
            if a.1 > 0 {
              Ordering::Less
            } else {
              Ordering::Greater
            }
          }
        }
        _ => (a.0).cmp(&b.0),
      }
    });

    let mut heights: Vec<i32> = vec![0];
    let mut last_record: i32 = 0; // the height of last record

    dbg!(&spread_buildings);
    for (x, h) in spread_buildings {
      // if is left-edge, push its height to heights
      // if is right-edge, remove its height from heights
      // and building.1 > 0 is left-edge, otherwise is right-edge
      if h >= 0 {
        heights.push(h);
      } else {
        // use multiset to speed-up
        let position: Option<usize> = heights.iter().position(|&x| x == -h);
        if position.is_some() {
          heights.swap_remove(position.unwrap());
        }
      }

      if let Some(&max_height) = heights.iter().max() {
        // a new point
        if max_height != last_record {
          res.push(vec![x, max_height]);
          last_record = max_height;
        }
      }
    }
    res
  }
}

#[cfg(test)]
mod get_skyline_test {
  use super::*;

  #[test]
  fn test_get_skyline() {
    // case 1
    let building_dimens = vec![
      vec![2, 9, 10],
      vec![3, 7, 15],
      vec![5, 12, 12],
      vec![15, 20, 10],
      vec![19, 24, 8],
    ];
    let res = Solution::get_skyline(building_dimens);
    let except_res = vec![
      vec![2, 10],
      vec![3, 15],
      vec![7, 12],
      vec![12, 0],
      vec![15, 10],
      vec![20, 8],
      vec![24, 0],
    ];
    assert_eq!(res, except_res);

    // case 2
    let building_dimens = vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]];
    let except_res = vec![vec![1, 3], vec![2, 0]];
    let res = Solution::get_skyline(building_dimens);
    assert_eq!(res, except_res);
  }
}
