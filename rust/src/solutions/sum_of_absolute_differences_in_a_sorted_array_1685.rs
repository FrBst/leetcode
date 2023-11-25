pub struct Solution {}

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0;nums.len()];

        for i in 1..nums.len() {
            res[0] += nums[i] - nums[0];
        }

        for i in 1..nums.len() {
            let distance = (nums[i] - nums[i - 1]);
            
            res[i] = res[i-1] + ((2 * i - nums.len()) as i32 * distance);
        }

        res
    }
}