struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut insert_point: usize = 0;
        while i < nums.len() && j < nums.len() {
            let item = nums[i];
            while j < nums.len() && nums[j] == item {
                if j - i < 2 {
                    nums[insert_point] = nums[j];
                    insert_point = insert_point + 1;
                }
                j = j + 1;
            }
            i = j;
        }
        return insert_point as i32;
    }
}

struct BetterSolution {}

impl BetterSolution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return len as i32;
        }

        let mut insert_point = 2;
        for i in 2..len {
            if nums[i] != nums[insert_point - 2] {
                nums[insert_point] = nums[i];
                insert_point = insert_point + 1;
            }
        }
        return insert_point as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut v1 = vec![1, 1, 1, 2, 2, 3];
        let mut v2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

        let len1 = Solution::remove_duplicates(&mut v1);
        assert_eq!(5, len1);
        assert_eq!(vec![1, 1, 2, 2, 3], v1[0..(len1 as usize)]);

        let len2 = Solution::remove_duplicates(&mut v2);
        assert_eq!(7, len2);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], v2[0..(len2 as usize)]);
    }

    #[test]
    fn test_better_remove_duplicates() {
        let mut v1 = vec![1, 1, 1, 2, 2, 3];
        let mut v2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

        let len1 = BetterSolution::remove_duplicates(&mut v1);
        assert_eq!(5, len1);
        assert_eq!(vec![1, 1, 2, 2, 3], v1[0..(len1 as usize)]);

        let len2 = BetterSolution::remove_duplicates(&mut v2);
        assert_eq!(7, len2);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], v2[0..(len2 as usize)]);
    }
}
