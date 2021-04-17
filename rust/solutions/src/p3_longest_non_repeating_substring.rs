use std::{cmp::max, collections::HashSet, convert::TryInto};

struct HashSetSolution {}

impl HashSetSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let chars: Vec<char> = s.chars().collect::<Vec<char>>();
        let len = chars.len();

        for i in 0..len {
            if len - i <= longest {
                break;
            }
            let mut seen = HashSet::new();
            for j in i..len {
                if seen.contains(&chars[j]) {
                    break;
                }
                if j - i + 1 > longest {
                    longest = j - i + 1;
                }
                seen.insert(chars[j]);
            }
        }
        return longest.try_into().unwrap();
    }
}

struct SlidingWindowSolution {}

impl SlidingWindowSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut seens = HashSet::with_capacity(len);
        let (mut longest, mut r) = (0, 0);
        for i in 0..len {
            while r < len && !seens.contains(&chars[r]) {
                seens.insert(chars[r]);
                r += 1;
            }
            longest = max(longest, r - i);
            seens.remove(&chars[i]);
        }
        return longest.try_into().unwrap();
    }
}

struct SlidingWindowNoHashSetSolution {}

impl SlidingWindowNoHashSetSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: &[u8] = s.as_bytes();
        let len = chars.len();
        let mut char_index: [i32; 128] = [-1; 128];
        let (mut longest, mut r) = (0, 0);
        for i in 0..len {
            while r < len && char_index[chars[r] as usize] == -1 {
                char_index[chars[r] as usize] = r as i32;
                r += 1;
            }
            longest = max(longest, r - i);
            char_index[chars[i] as usize] = -1;
        }
        return longest.try_into().unwrap();
    }
}

/* This solution is pretty clever.
 * impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last: [i32; 128] = [-1; 128];
        let mut left = -1;
        let mut ans = 0;
        for (i, v) in s.chars().enumerate() {
            left = max(left, last[v as usize]);
            last[v as usize] = i as i32;
            ans = max(ans, (i as i32) - left);
        }
        return ans;
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            HashSetSolution::length_of_longest_substring("abcabcbb".into()),
            3
        );
        assert_eq!(HashSetSolution::length_of_longest_substring(" ".into()), 1);
        assert_eq!(
            HashSetSolution::length_of_longest_substring("bbbbb".into()),
            1
        );
        assert_eq!(
            HashSetSolution::length_of_longest_substring("pwwkew".into()),
            3
        );
    }

    #[test]
    fn test_sliding_window() {
        assert_eq!(
            SlidingWindowSolution::length_of_longest_substring("abcabcbb".into()),
            3
        );
        assert_eq!(SlidingWindowSolution::length_of_longest_substring(" ".into()), 1);
        assert_eq!(
            SlidingWindowSolution::length_of_longest_substring("bbbbb".into()),
            1
        );
        assert_eq!(
            SlidingWindowSolution::length_of_longest_substring("pwwkew".into()),
            3
        );
    }

    #[test]
    fn test_sliding_window2() {
        assert_eq!(
            SlidingWindowNoHashSetSolution::length_of_longest_substring("abcabcbb".into()),
            3
        );
        assert_eq!(SlidingWindowNoHashSetSolution::length_of_longest_substring(" ".into()), 1);
        assert_eq!(
            SlidingWindowNoHashSetSolution::length_of_longest_substring("bbbbb".into()),
            1
        );
        assert_eq!(
            SlidingWindowNoHashSetSolution::length_of_longest_substring("pwwkew".into()),
            3
        );
    }
}
