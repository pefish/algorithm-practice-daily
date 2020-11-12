/// 给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数大于⌊ n/2 ⌋的元素。

/// 你可以假设数组是非空的，并且给定的数组总是存在多数元素。

/// 示例1:

/// 输入: [3,2,3]
/// 输出: 3
/// 示例2:

/// 输入: [2,2,1,1,1,2,2]
/// 输出: 2


use std::collections::HashMap;

struct Solution {}

impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut result_map: HashMap<i32, i32> = HashMap::new();
    for &num in &nums {
      let new_value;
      if result_map.get(&num).is_none() {
        new_value = 1;
      } else {
        new_value = result_map.get(&num).unwrap() + 1;
      }
      result_map.insert(num, new_value);
      if new_value > (nums.len() / 2) as i32 {
        return num;
      }
    }
    return 0;
  }
}

fn main () {
  println!("{}", Solution::majority_element(vec![1, 2, 1]));
}
