//
// 字符串压缩。利用字符重复出现的次数，编写一种方法，实现基本的字符串压缩功能。比如，字符串aabcccccaaa会变为a2b1c5a3。若“压缩”后的字符串没有变短，则返回原先的字符串。你可以假设字符串中只包含大小写英文字母（a至z）。
//
// 示例1:
//
// 输入："aabcccccaaa"
// 输出："a2b1c5a3"
// 示例2:
//
// 输入："abbccd"
// 输出："abbccd"
// 解释："abbccd"压缩后为"a1b2c2d1"，比原字符串长度更长。
//

#[derive(Debug)]
struct Solution {}


impl Solution {
  pub fn compress_string(s: String) -> String {
    if s.len() == 0 {
      return s;
    }
    let mut chars = s.chars();    
    let first_char = chars.next().unwrap();
    let mut current: char = first_char;
    let mut current_num: i32 = 1;
    let mut result: String = first_char.to_string();
    for char_ in chars {
      // println!("current: {}, current_num: {}, char: {:?}", current, current_num, char_);
      if char_ != current {
        result = result + &current_num.to_string();
        result = result + &char_.to_string();
        current_num = 1;
      } else {
        current_num = current_num + 1;
      }
      current = char_;
      // println!("{:?}", result);
    }
    result = result + &current_num.to_string();
    // println!("{:?}", result);
    if result.len() >= s.len() {
      return s;
    }
    return result;
  }
}

fn main() {
  let str_ = String::from("aabcccccaa");
  println!("{:?}", Solution::compress_string(str_));
}
