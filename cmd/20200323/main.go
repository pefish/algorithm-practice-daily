//
//给定一个带有头结点 head 的非空单链表，返回链表的中间结点。
//
//如果有两个中间结点，则返回第二个中间结点。
//
//
//
//示例 1：
//
//输入：[1,2,3,4,5]
//输出：此列表中的结点 3 (序列化形式：[3,4,5])
//返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
//注意，我们返回了一个 ListNode 类型的对象 ans，这样：
//ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.
//示例 2：
//
//输入：[1,2,3,4,5,6]
//输出：此列表中的结点 4 (序列化形式：[4,5,6])
//由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
//
//
//提示：
//
//给定链表的结点数介于 1 和 100 之间。
//

package main

import (
	"fmt"
	"time"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
	// 先计算长度
	node := &ListNode{
		Val:  0,
		Next: head,
	}
	length := 0
	for node.Next != nil {
		node = node.Next
		length++
	}

	// 计算中间元素
	var startIndex int = length / 2
	fmt.Println("startIndex:", startIndex)
	if startIndex == 0 {
		return head
	}
	counter := 0
	for head.Next != nil {
		head = head.Next
		counter++
		if startIndex == counter {
			return head
		}
	}
	return nil
}

func main() {
	t := time.Now()

	result := middleNode(&ListNode{
		Val: 1,
		Next: nil,
	})
	fmt.Println(result)
	fmt.Printf("%d ms \n", time.Now().Sub(t).Microseconds())
}
