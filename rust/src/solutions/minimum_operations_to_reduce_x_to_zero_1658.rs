pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        
        let target = nums.iter().sum::<i32>() - x;

        if target == 0 {
            return nums.len() as i32;
        }

        let mut cur_sum = 0;
        let mut max_len = 0;

        let mut left = 0;
        for right in 0..nums.len() {
            cur_sum += nums[right];
            while left <= right && cur_sum > target {
                cur_sum -= nums[left];
                left += 1;
            }

            if (cur_sum == target) {
                max_len = max_len.max(right - left + 1);
            }
        }

        if max_len == 0 { -1 } else { (nums.len() - max_len) as i32 }
    }
}