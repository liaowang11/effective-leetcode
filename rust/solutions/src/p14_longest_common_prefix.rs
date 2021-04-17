struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let shortest: &str = strs.iter().min_by_key(|s| s.len()).unwrap();
        let (mut left, mut right) = (0, shortest.len());
        while left <= right {
            let mid = (right + left) / 2;
            if Self::is_common_prefix(&shortest[0..mid], &strs) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return shortest[0..right].to_string();
    }

    fn is_common_prefix(prefix: &str, strs: &Vec<String>) -> bool {
        strs.iter().all(|s| s.starts_with(prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisearch() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
