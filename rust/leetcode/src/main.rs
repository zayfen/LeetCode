mod dp;
mod hash_table;

fn main() {
  println!("{}", dp::maximum_subarray::max_sub_array(vec!(-2, 1, -3, 4, -1, 2, 1, -5, 4)));


  println!("==========================");
  let mut result: Vec<i32> = Vec::new();
  // let nums = &[-2, -1, 0, 0, 1, 2]; //
  // let nums = &[1, 0, -1, 0, -2, 2];
  let nums = &[-3, -2, -1, 0, 0, 1, 2, 3];
  let mut nums = nums.clone();
  nums.sort();
  let nums  = nums.clone();
  let mut store: Vec<Vec<i32>> = Vec::new();
  
  hash_table::four_sum::n_sum(&nums, 0, 4, result, &mut store);
  println!("store: {:?} ", store);
}
