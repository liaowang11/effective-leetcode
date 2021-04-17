struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let ans = &mut vec![];
        let cur = &mut vec![];
        Self::dfs(&candidates, target, 0, cur, ans);
        return ans.clone();
    }

    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        idx: usize,
        cur: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if idx == candidates.len() {
            return;
        }

        if target == 0 {
            ans.push(cur.clone());
            return;
        }

        Self::dfs(candidates, target, idx + 1, cur, ans);

        if target >= candidates[idx] {
            cur.push(candidates[idx]);
            Self::dfs(candidates, target - candidates[idx], idx, cur, ans);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7).sort(),
            vec![vec![2, 2, 3], vec![7]].sort()
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 7).sort(),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]].sort()
        );
        assert_eq!(Solution::combination_sum(vec![2], 1), (vec![] as Vec<Vec<i32>>));
        assert_eq!(Solution::combination_sum(vec![1], 1), vec![vec![1]]);
        assert_eq!(Solution::combination_sum(vec![1], 2), vec![vec![1, 1]]);
    }
}
