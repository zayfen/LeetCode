/**
Maximum Subarray

Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

Example:

Input: [-2,1,-3,4,-1,2,1,-5,4],
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
Follow up:

If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
**/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // if curr_num + prev_num > curr_num ,then make nums[curr] = curr_nums + prev_nums else nums[curr] = nums[curr]
    let len_nums = nums.len();
    let mut result: Vec<i32> = Vec::new();
    result.push(*nums.get(0).unwrap());

    for i in 1..len_nums {
        let curr_num = *nums.get(i).unwrap();
        let prev_num = result.get(result.len() - 1).unwrap();

        if curr_num + prev_num >= curr_num {
            result.push(curr_num + prev_num);
        } else {
            result.push(curr_num);
        }
    }
    println!("{:?}", nums);
    println!("{:?}", result);
    *result.iter().max().unwrap()
}

pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
    let len_nums = nums.len();

    let mut result = nums;

    for i in 1..len_nums {
        let curr_num = *result.get(i).unwrap();
        let prev_num = *result.get(i - 1).unwrap();

        if curr_num + prev_num >= curr_num {
            *result.get_mut(i).unwrap() = curr_num + prev_num;
        }
    }

    println!("{:?}", result);

    *result.iter().max().unwrap()
}

#[cfg(test)]
mod dp_tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(6, max_sub_array(vec!(-2, 1, -3, 4, -1, 2, 1, -5, 4)));
    }

    #[test]
    fn test_max_sub_array_v2() {
        assert_eq!(6, max_sub_array_v2(vec!(-2, 1, -3, 4, -1, 2, 1, -5, 4)));
    }
}
