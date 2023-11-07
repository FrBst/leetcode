pub struct Solution {}

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {

        let mut max = 0;

        for x in left {
            max = max.max(x);
        }

        for x in right {
            max = max.max(n - x);
        }

        max
    }
}