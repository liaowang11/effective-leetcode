struct MySolution {}

impl MySolution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let len = nums.len();
        if len == 1 && nums[0] == target {
            return 0;
        }
        let mut rotate_point = 0;
        while rotate_point < len - 1 && nums[rotate_point + 1] > nums[rotate_point] {
            rotate_point = rotate_point + 1;
        }
        let head = Self::binary_search(&nums[0..rotate_point + 1], target);
        if head != -1 {
            return head;
        }
        let tail = Self::binary_search(&nums[rotate_point + 1..len], target);
        if tail != -1 {
            return (rotate_point as i32) + tail + 1;
        }
        return -1;
    }

    fn  binary_search(nums: &[i32], target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut i: i32 = 0;
        let mut j: i32 = (nums.len() - 1) as i32;
        while i <= j {
            let pivot = (j + i + 1) / 2;
            let pivot_num = nums[pivot as usize];
            if pivot_num == target {
                return pivot as i32;
            } else if pivot_num < target {
                i = pivot + 1;
            } else {
                j = pivot - 1;
            }
        }
        return -1;
    }
    }

struct OfficialSolution {}

impl OfficialSolution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return -1;
        }
        if len == 1 {
            return {
                if nums[0] == target {
                    0
                } else {
                    -1
                }
            };
        }

        let (mut l, mut r) = (0, len - 1);

        while l <= r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            // 前面是有序的
            if nums[0] <= nums[mid] {
                if nums[0] <= target && target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[len - 1] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(3, MySolution::binary_search(&vec![1, 2, 5, 7, 9, 11], 7));
        assert_eq!(3, MySolution::binary_search(&vec![1, 2, 5, 6, 7, 9, 11], 6));
        assert_eq!(
            -1,
            MySolution::binary_search(&vec![1, 2, 5, 6, 7, 9, 11], 8)
        );
        assert_eq!(-1, MySolution::binary_search(&vec![1, 2, 5, 7, 9, 11], 6));
        assert_eq!(0, MySolution::binary_search(&vec![0, 1, 2], 0));
    }

    #[test]
    fn test_search() {
        assert_eq!(4, MySolution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, MySolution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(-1, MySolution::search(vec![1], 0));
    }
}
