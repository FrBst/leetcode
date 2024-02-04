pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob = 0;
        let mut norob = 0;

        for x in nums {
            let new_rob = norob + x;
            let new_norob = rob.max(norob);
            rob = new_rob;
            norob = new_norob;
        }

        rob.max(norob)
    }
}