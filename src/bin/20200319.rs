//
//给定一个包含大写字母和小写字母的字符串，找到通过这些字母构造成的最长的回文串。
//
//在构造过程中，请注意区分大小写。比如 "Aa" 不能当做一个回文字符串。
//
//注意:
//假设字符串的长度不会超过 1010。
//
//示例 1:
//
//输入:
//"abccccdd"
//
//输出:
//7
//
//解释:
//我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
//

#[derive(Debug)]
struct Solution {}

use std::collections::HashMap;

impl Solution {
  pub fn longest_palindrome(s: String) -> i32 {
    let mut result: i32 = 0;
    let chars_ = s.chars();
    let mut counts: HashMap<char, i32> = HashMap::new();
    for char_ in chars_ {
      counts.insert(char_, counts.get(&char_).unwrap_or(&0) + 1);
    }
    for count in counts {
      let v = count.1;
      result += v / 2 * 2;
      if v % 2 == 1 && result % 2 == 0 {
        result += 1;
      }
    }
    return result;
  }
}

fn main() {
  println!("{:?}", Solution::longest_palindrome(String::from("abccccdd")));
}
