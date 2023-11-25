pub struct Solution {}

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort_unstable();
        let n = piles.len() / 3;
        piles.into_iter().skip(n).step_by(2).sum()
    }
}