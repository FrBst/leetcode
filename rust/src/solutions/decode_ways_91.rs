pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        if s[0] == b'0' {
            return 0;
        }

        let mut dp = vec![0;s.len()];
        dp[0] = 1;

        for i in 1..s.len() {
            if s[i-1] != b'0' && (s[i-1] < b'2' || s[i-1] <= b'2' && s[i] <= b'6') {
                dp[i] += if i > 1 { dp[i-2] } else { 1 };
            }

            if s[i] != b'0' {
                dp[i] += dp[i-1];
            }
        }

        dp[s.len() - 1]
    }
}

#[test]
fn test() {
    let result = Solution::num_decodings("1201234".to_string());
    assert_eq!(result, 3);
}