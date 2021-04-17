struct Solution {}
/// TODO: don't convert to str..
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            Self::is_palindrome_str(&x.to_string())
        }
    }
    fn is_palindrome_str(s: &str) -> bool {
        let len = s.len();
        let chars = s.as_bytes();
        let (mut left, mut right) = if len % 2 == 0 {
            ((len / 2 - 1) as i32, len / 2)
        } else {
            ((len / 2) as i32, len / 2)
        };
        while left >= 0 && right <= len - 1 {
            if chars[left as usize] != chars[right] {
                return false;
            }
            left -= 1;
            right += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_number() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
