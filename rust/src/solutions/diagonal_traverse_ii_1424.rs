use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {

        let mut ans = Vec::new();
        
        let mut queue = VecDeque::new();
        queue.push_back((0,0));

        while let Some((row,col)) = queue.pop_front() {
            ans.push(nums[row][col]);

            if col == 0 && nums.len() > row + 1 {
                queue.push_back((row + 1, col));
            }
            if nums[row].len() > col + 1 {
                queue.push_back((row, col + 1));
            }
        }

        ans
    }
}