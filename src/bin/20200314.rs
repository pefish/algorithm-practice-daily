//
// 给定一个无序的整数数组，找到其中最长上升子序列的长度。
//
// 示例:
// 输入: [10,9,2,5,3,7,101,18]
// 输出: 4
// 解释: 最长的上升子序列是 [2,3,7,101]，它的长度是 4。
//
// 说明:
// 可能会有多种最长上升子序列的组合，你只需要输出对应的长度即可。
// 你算法的时间复杂度应该为 O(n2) 。
//

#[derive(Debug)]
struct Solution {}

impl Solution {
  pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
      return 0;
    }
    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = 1;
    let mut result = 1;
    for i in 1..n {
      let mut max_val = 0;
      for j in 0..i {
        if nums[j] < nums[i] {
          max_val = max_val.max(dp[j]);
        }
      }
      dp[i] = max_val + 1;
      result = result.max(dp[i]);
    }

    return result;
  }
}

fn main() {
  let nums = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
  println!("{:?}", Solution::length_of_lis(nums));
}
