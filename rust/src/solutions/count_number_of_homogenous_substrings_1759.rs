pub struct Solution {}

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let modulo: i64 = 1_000_000_000 + 7;
        let mut count: i64 = 0;

        let s = s.as_bytes();
        let mut current_count: i64 = 1;
        for i in 1..s.len() + 1 {
            if i < s.len() && s[i] == s[i-1] {
                current_count += 1;
            } else {
                count += (current_count * (current_count + 1)) / 2;
                count %= modulo;
                current_count = 1;
            }
        }

        count as i32
    }
}