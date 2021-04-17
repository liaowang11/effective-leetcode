struct Solution {

}

impl Solution {
    pub fn is_ugly_number(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        for i in &[2,3,4] {
            while n % i == 0 {
                n /= i;
            }
        }
        if n == 1 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ugly_number() {
        assert_eq!(Solution::is_ugly_number(6), true);
        assert_eq!(Solution::is_ugly_number(8), true);
        assert_eq!(Solution::is_ugly_number(14), false);
        assert_eq!(Solution::is_ugly_number(1), true);
    }
}
