pub struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut reversed: u32 = 0;
        let mut x = x;

        for _ in 0..32 {
            reversed = (reversed << 1) | (x & 1);
            x = x >> 1;
        }

        reversed
    }
}