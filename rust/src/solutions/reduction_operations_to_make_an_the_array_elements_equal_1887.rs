pub struct Solution {}

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums.reverse();

        let mut count = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                count += i as i32;
            }
        }

        count
    }
}