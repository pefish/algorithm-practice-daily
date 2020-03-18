//
//矩形以列表 [x1, y1, x2, y2] 的形式表示，其中 (x1, y1) 为左下角的坐标，(x2, y2) 是右上角的坐标。
//
//如果相交的面积为正，则称两矩形重叠。需要明确的是，只在角或边接触的两个矩形不构成重叠。
//
//给出两个矩形，判断它们是否重叠并返回结果。
//
// 
//
//示例 1：
//
//输入：rec1 = [0,0,2,2], rec2 = [1,1,3,3]
//输出：true
//示例 2：
//
//输入：rec1 = [0,0,1,1], rec2 = [1,0,2,1]
//输出：false
//
//提示：
//
//两个矩形 rec1 和 rec2 都以含有四个整数的列表的形式给出。
//矩形中的所有坐标都处于 -10^9 和 10^9 之间。
//x 轴默认指向右，y 轴默认指向上。
//你可以仅考虑矩形是正放的情况。
//


#[derive(Debug)]
struct Solution {}


impl Solution {
  pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    return (rec1.get(0).unwrap() < rec2.get(2).unwrap()) && (rec1.get(1).unwrap() < rec2.get(3).unwrap()) && (rec1.get(2).unwrap() > rec2.get(0).unwrap()) && (rec1.get(3).unwrap() > rec2.get(1).unwrap())
  }
}

fn main() {
  println!("{:?}", Solution::is_rectangle_overlap(vec![0,0,2,2], vec![1,1,3,3]));
}
