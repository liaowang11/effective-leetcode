use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut cache = HashMap::with_capacity(8);
        Self::generate(n, &mut cache)
    }

    fn generate(n: i32, cache: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        if let Some(result) = cache.get(&n) {
            return result.clone();
        }

        let mut result = vec![];
        if n == 0 {
            result.push("".to_string());
        } else {
            for i in 0..n {
                for left in Self::generate(i, cache) {
                    for right in Self::generate(n - i - 1, cache) {
                        let mut new = String::new();
                        new.push('(');
                        new.push_str(&left);
                        new.push(')');
                        new.push_str(&right);
                        result.push(new);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let mut case1 = Solution::generate_parenthesis(3);
        case1.sort();
        assert_eq!(
            case1,
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
