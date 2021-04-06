package solutions

import ()

func removeDuplicates(nums []int) int {
	insertPoint := 2
	for i := 2; i < len(nums); i++ {
		if nums[i] != nums[insertPoint-2] {
			nums[insertPoint] = nums[i]
			insertPoint++
		}
	}
	return insertPoint
}
