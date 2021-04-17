use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        Self::combinations(&digits.as_bytes())
    }

    fn combinations(digits: &[u8]) -> Vec<String> {
        const LETTERS: &[&str] = &["", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        if digits.is_empty() {
            return Vec::new();
        }
        let len = digits.len();
        let head_letters = LETTERS[(digits[0] as usize) - ('1' as usize)];
        let mut result = Vec::new();
        if head_letters.is_empty() {
            return Self::combinations(&digits[1..len]);
        } else {
            for c in head_letters.as_bytes() {
                let combs = Self::combinations(&digits[1..len]);
                if combs.is_empty() {
                    let mut new_str = String::new();
                    new_str.push(*c as char);
                    result.push(new_str);
                } else {
                    for comb in combs {
                        let mut new_str = String::new();
                        new_str.push(*c as char);
                        new_str.push_str(&comb);
                        result.push(new_str);
                    }
                }
            }
        }
        return result;
    }
}

struct QueueSolution {}

impl QueueSolution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        const LETTERS: &[&str] = &["", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut queue = VecDeque::new();
        queue.push_back("".to_string());
        for d in digits.chars() {
            for _ in 0..queue.len() {
                let head: String = queue.pop_front().unwrap();
                for c in LETTERS[(d as usize) - ('1' as usize)].chars() {
                    let mut new_str = head.clone();
                    new_str.push(c);
                    queue.push_back(new_str);
                }
            }
        }
        queue.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        let empty: Vec<String> = Vec::new();
        assert_eq!(Solution::letter_combinations("".to_string()), empty);
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_queue_combinations() {
        assert_eq!(
            QueueSolution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        let empty: Vec<String> = Vec::new();
        assert_eq!(QueueSolution::letter_combinations("".to_string()), empty);
        assert_eq!(
            QueueSolution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
