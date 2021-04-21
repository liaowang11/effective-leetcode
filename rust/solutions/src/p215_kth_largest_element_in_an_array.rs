use rand::{thread_rng, Rng};
struct Solution {}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        let length = nums.len();
        return Self::quick_select(&mut nums, length - (k as usize));
    }
    fn quick_select(nums: &mut [i32], index: usize) -> i32 {
        let length = nums.len();
        if length == 1 {
            return nums[0];
        }
        let pivot = Self::partition(nums);
        println!("after partition {:?} at pivot nums[{:?}]={:?} index={:?}", nums, pivot, nums[pivot], index);
        if pivot == index {
            return nums[pivot];
        }
        if pivot > index {
            println!("Searching left {:?} for {:?}", &nums[0..pivot], index);
            return Self::quick_select(&mut nums[0..pivot], index);
        } else {
            println!(
                "Searching right {:?} for {:?}",
                &nums[(pivot + 1)..length],
                index - pivot - 1
            );
            return Self::quick_select(&mut nums[(pivot + 1)..length], index - pivot - 1);
        }
    }
    fn partition(nums: &mut [i32]) -> usize {
        let length = nums.len();
        let pivot = thread_rng().gen_range(0..length);
        let pivot_num = nums[pivot];
        nums.swap(pivot, length - 1);
        let mut i = 0;
        for j in 0..length {
            if nums[j] < pivot_num {
                nums.swap(j, i);
                i += 1;
            }
        }
        nums.swap(i, length - 1);
        return i;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,1,5,6,4], 2),
                   5)
    }
}
