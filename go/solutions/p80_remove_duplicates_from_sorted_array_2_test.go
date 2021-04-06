package solutions

import (
	"testing"
)

func TestRemoveDuplicates(t *testing.T) {
	test1 := []int{1, 1, 1, 2, 2, 3}
	len1 := removeDuplicates(test1)
	if len1 != 5 {
		t.Fatalf("Error %v", len1)
	}
}
