struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return false;
        }
        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                ')' | ']' | '}' => {
                    if let Some(pair) = stack.pop() {
                        if pair != Self::get_pair(c) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        if !stack.is_empty() {
            return false;
        }
        return true;
    }
    fn get_pair(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => panic!("invalid char")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)])".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
