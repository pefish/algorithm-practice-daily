//
//给你一份『词汇表』（字符串数组） words 和一张『字母表』（字符串） chars。
//
//假如你可以用 chars 中的『字母』（字符）拼写出 words 中的某个『单词』（字符串），那么我们就认为你掌握了这个单词。
//
//注意：每次拼写时，chars 中的每个字母都只能用一次。
//
//返回词汇表 words 中你掌握的所有单词的 长度之和。
//
//
//
//示例 1：
//
//输入：words = ["cat","bt","hat","tree"], chars = "atach"
//输出：6
//解释： 
//可以形成字符串 "cat" 和 "hat"，所以答案是 3 + 3 = 6。
//示例 2：
//
//输入：words = ["hello","world","leetcode"], chars = "welldonehoneyr"
//输出：10
//解释：
//可以形成字符串 "hello" 和 "world"，所以答案是 5 + 5 = 10。
//

#[derive(Debug)]
struct Solution {}


impl Solution {
  pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut result: i32 = 0;
    for word in words {
      let mut a = true;
      let mut new_char = chars.clone();
      for char_ in word.chars() {
        let find_result = new_char.find(char_);
        if find_result.is_none() {
          a = false;
          break
        }
        new_char.remove(find_result.unwrap());
      }
      // println!("{}, {}", word, a);
      if a {
        result += word.len() as i32;
      }
    }
    return result;
  }
}

fn main() {
  println!("{:?}", Solution::count_characters(vec!["cat".to_string(),"bt".to_string(),"hat".to_string(),"tree".to_string()], String::from("atach")));
}
