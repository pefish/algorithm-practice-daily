//
//给定一个整数数组 nums和一个目标值 target，请你在该数组中找出和为目标值的那两个整数，并返回他们的数组下标。
//
//你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
//
//示例:
//
//给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
//

#[derive(Debug)]
struct Solution {}

use std::collections::HashMap;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map_: HashMap<i32, i32> = HashMap::new();
    for index in 0..nums.len() {
      map_.insert(*nums.get(index).unwrap(), index as i32);
    }
    for index in 0..nums.len() {
      let temp = target - nums.get(index).unwrap();
      let other_index = map_.get(&temp);
      if other_index.is_some() && index as i32 != *other_index.unwrap() {
        return vec!(index as i32, *other_index.unwrap());
      }
    }
    return vec!();
  }
}

fn main() {
  println!("{:?}", Solution::two_sum(vec!(2, 7, 11, 15), 9));
}
