pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];
        let mut sum = 0;
        
        for num in nums {
            let num = num as usize;
            // Better memory usage than `if ... { sum += ... }`
            sum += if count[num] > 0 { count[num] } else { 0 };
            count[num] += 1;
        }

        sum
    }
}