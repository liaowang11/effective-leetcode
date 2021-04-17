use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;
        for num in nums {
            if num_set.contains(&(num - 1)) {
                continue;
            }
            let mut start = num;
            while num_set.contains(&start) {
                start += 1;
            }
            if start - num > longest {
                longest = start - num;
            }
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100,4,200,1,3,2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }
}
