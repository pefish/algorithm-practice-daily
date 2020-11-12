package main

import (
	"fmt"
	"time"
)

// 给定一个非负整数数组 A， A 中一半整数是奇数，一半整数是偶数。
//
//对数组进行排序，以便当 A[i] 为奇数时，i也是奇数；当A[i]为偶数时， i 也是偶数。
//
//你可以返回任何满足上述条件的数组作为答案。
//
//
//
//示例：
//
//输入：[4,2,5,7]
//输出：[4,5,2,7]
//解释：[4,7,2,5]，[2,5,4,7]，[2,7,4,5] 也会被接受。
//
//
//提示：
//
//2 <= A.length <= 20000
//A.length % 2 == 0
//0 <= A[i] <= 1000
//

func sortArrayByParityII(A []int) []int {
	nextResult := 0
	point1 := 0
	point2 := 1
	for range A {
		if A[point1] % 2 == nextResult {
			point1 ++
			point2 = point1 + 1
			nextResult ^= 1
			continue
		}
		for point2 < len(A) {
			if A[point2] % 2 == nextResult {
				A[point1], A[point2] = A[point2], A[point1]
				point1 ++
				point2 = point1 + 1
				nextResult ^= 1
				break
			}
			point2++
		}
	}
	return A
}

func main() {
	t := time.Now()
	result := sortArrayByParityII([]int{4,2,5,7})
	fmt.Println(result)
	fmt.Printf("%d ms \n", time.Now().Sub(t).Microseconds())
}
