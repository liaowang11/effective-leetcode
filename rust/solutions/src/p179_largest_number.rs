use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.is_empty() {
            return String::new();
        }
        if nums.iter().all(|x| *x == 0) {
            return "0".to_string();
        }
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable_by(|x, y| -> Ordering {
            let ceil_x = Self::get_ceiling(*x);
            let ceil_y = Self::get_ceiling(*y);
            (*y as u64 * ceil_x as u64 + *x as u64).cmp(&(*x as u64 * ceil_y as u64 + *y as u64))
            // (*x * ceil_y * 10 + *y).cmp(&(*y * ceil_x * 10 + *x))
        });
        let mut result = String::new();
        let mut head = true;
        for num in sorted_nums.iter() {
            result.push_str(&num.to_string());
            head = false;
        }
        result
    }
    fn get_ceiling(x: i32) -> i32 {
        let mut ceil = 10;
        while ceil <= x {
            ceil *= 10;
        }
        ceil
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ceiling() {
        assert_eq!(Solution::get_ceiling(120), 1000);
        assert_eq!(Solution::get_ceiling(9), 10);
        assert_eq!(Solution::get_ceiling(10), 100);
        assert_eq!(Solution::get_ceiling(111311), 1000000);
        assert_eq!(Solution::get_ceiling(1113), 10000);
    }

    #[test]
    pub fn test_largest_number() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
        assert_eq!(Solution::largest_number(vec![111311, 1113]), "1113111311");
        assert_eq!(Solution::largest_number(vec![0, 0]), "0");
        assert_eq!(
            Solution::largest_number(vec![999999998, 999999997, 999999999]),
            "999999999999999998999999997"
        );
        assert_eq!(Solution::largest_number(vec![1]), "1");
        assert_eq!(Solution::largest_number(vec![10]), "10");
    }
}
