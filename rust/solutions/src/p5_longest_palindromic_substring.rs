struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = String::new();
        let chars = s.as_bytes();
        for i in 0..chars.len() {
            let sub1 = Self::find_palindrome(chars, i, i);
            if sub1.len() > longest.len() {
                longest = sub1;
            }
            let sub2 = Self::find_palindrome(chars, i, i + 1);
            if sub2.len() > longest.len() {
                longest = sub2;
            }
        }
        return longest;
    }

    /// 这里返回的String，比较浪费内存
    pub fn find_palindrome(chars: &[u8], left: usize, right: usize) -> String {
        let (mut i, mut j) = (left as i32, right);
        let len = chars.len();
        while i >= 0 && j <= (len - 1) && chars[i as usize] == chars[j] {
            i -= 1;
            j += 1;
        }
        return String::from_utf8_lossy(&chars[((i + 1) as usize)..=(j - 1)]).into_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pal() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a");
    }
}
