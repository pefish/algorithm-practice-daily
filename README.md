
## 数据结构与算法每日一练

## 时间复杂度

时间复杂度是对一个算法在运行过程中花费时间大小的一个量度，反映的是一个趋势

例如：

```C++
for(m=1; m<n; m++)
{
    i = 1;
    while(i<n)
    {
        i = i * 2;
    }
}
```

复杂度就是 n * O(logN)，也就是了O(nlogN)

## 空间复杂度

空间复杂度是对一个算法在运行过程中临时占用存储空间大小的一个量度，同样反映的是一个趋势

例如：

```C++
int[] m = new int[n]
for(i=1; i<=n; ++i)
{
   j = i;
   j++;
}
```

这段代码中，第一行new了一个数组出来，这个数据占用的大小为n，这段代码的2-6行，虽然有循环，但没有再分配新的空间，因此，这段代码的空间复杂度主要看第一行，即O(n)

## 示例

**src/bin/20200313.rs**
```rust
/// 给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数大于 ⌊ n/2 ⌋ 的元素。

/// 你可以假设数组是非空的，并且给定的数组总是存在多数元素。

/// 示例 1:

/// 输入: [3,2,3]
/// 输出: 3
/// 示例 2:

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

```

## 执行

```
cargo run --bin 20200313
```
