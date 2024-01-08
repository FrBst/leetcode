pub struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max = s.as_bytes().iter().skip(1).filter(|&&b| b == b'1').count() as i32;

        let mut current = max;
        for &c in s.as_bytes() {
            current += (if c == b'1' {-1} else {1});
            max = current.max(max);
        }

        max
    }
}