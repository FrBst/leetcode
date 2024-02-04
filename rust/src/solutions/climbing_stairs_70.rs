pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {

        let (mut a, mut b, mut res) = (0, 1, 0);

        for i in 1..=n {
            res = a + b;
            a = b;
            b = res;
        }

        res
    }
}
