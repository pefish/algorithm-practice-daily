//
//给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
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

package main

import (
	"fmt"
	"time"
)

func twoSum(nums []int, target int) []int {
	map_ := map[int]int{}
	for index, num := range nums {
		map_[num] = index
	}
	for index, num := range nums {
		result := target - num
		if val, ok := map_[result]; ok && val != index {
			return []int{index, val}
		}
	}
	return []int{}
}

func main() {
	t := time.Now()

	result := twoSum([]int{3, 2, 4}, 6)
	fmt.Println(result)
	fmt.Printf("%d ms \n", time.Now().Sub(t).Microseconds())
}
