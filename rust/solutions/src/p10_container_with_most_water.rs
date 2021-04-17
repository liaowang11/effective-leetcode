use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut max = 0;
        while i < j {
            let area = Self::area(&height, i, j);
            if area > max {
                max = area;
            }
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max
    }
    fn area(height: &[i32], left: usize, right: usize) -> i32 {
        let lowest = cmp::min(height[left], height[right]);
        lowest * ((right - left) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(Solution::max_area(vec![1,1]), 1);
        assert_eq!(Solution::max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(Solution::max_area(vec![1,2,1]), 2);
    }
}
