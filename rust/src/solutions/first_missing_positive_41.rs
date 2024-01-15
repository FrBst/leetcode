pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        
        let n = nums.len();

        for i in 0..n {
            if nums[i] <= 0 {
                nums[i] = (n + 1) as i32;
            }
        }

        for i in 0..n {
            let x = nums[i].abs() as usize - 1;

            if x >= n {
                continue;
            }

            if (nums[x] > 0) {
                nums[x] = - nums[x];
            }
        }

        for i in 0..n {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }
        
        return (n as i32) + 1;
    }
}

#[test]
fn test() {
    Solution::first_missing_positive(vec![1,2,0]);
}