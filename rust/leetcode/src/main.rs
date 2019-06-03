mod dp;
mod hash_table;
mod array;
mod string;

fn main() {
  println!("{}", dp::maximum_subarray::max_sub_array(vec!(-2, 1, -3, 4, -1, 2, 1, -5, 4)));


  println!("==========================");
  let mut result: Vec<i32> = Vec::new();
  // let nums = &[-2, -1, 0, 0, 1, 2]; //
  // let nums = &[1, 0, -1, 0, -2, 2];
  // let nums = &[-3, -2, -1, 0, 0, 1, 2, 3];
  let nums = &[0, 0];
  let mut nums = nums.clone();
  nums.sort();
  let nums  = nums.clone();
  let mut store: Vec<Vec<i32>> = Vec::new();

  println!(">>>>>>>>>>>>>>>>>> 0000000 >>>>>>>>>");
  println!("sorted numbers: {:?}", nums);
  hash_table::four_sum::n_sum_v2(&nums, 0, 4, result, &mut store);
  println!("store: {:?} ", store);

  println!(">>>>>>>>>>>>>>>>>>> 11111111 >>>>>>>>>>>>>>>>>>>");

  let nums = &[-494,-487,-471,-470,-465,-462,-447,-445,-441,-432,-429,-422,-406,-398,-397,-364,-344,-333,-328,-307,-302,-293,-291,-279,-269,-269,-268,-254,-198,-181,-134,-127,-115,-112,-96,-94,-89,-58,-58,-58,-44,-2,-1,43,89,92,100,101,106,106,110,116,143,156,168,173,192,231,248,256,281,316,321,327,346,352,353,355,358,365,371,410,413,414,447,473,473,475,476,481,491,498];
  let mut nums = nums.clone();
  nums.sort();
  let nums  = nums.clone();
  let mut store: Vec<Vec<i32>> = Vec::new();
  let mut result: Vec<i32> = Vec::new();
  
  hash_table::four_sum::n_sum_v2(&nums, 8511, 4, result, &mut store);
  println!("store: {:?} ", store);

  println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
}
