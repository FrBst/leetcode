pub struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {

        let MOD = 1_000_000_007;
        let corridor: &[u8] = corridor.as_bytes();
        let mut res: i64 = 1;

        let mut seats = 0;
        let mut last_group_end: i64  = -1;

        for i in 0..corridor.len() {
            if (corridor[i] == b'S') {
                seats += 1;

                if seats == 2 {
                    seats = 0;
                    last_group_end = i as i64;
                } else
                if seats == 1 && last_group_end != -1 {
                    res *= i as i64 - last_group_end;
                    res %= MOD;
                }
            }
        }

        if seats == 1 || last_group_end == -1 {
            0
        } else {
            res as i32
        }
    }
}

#[test]
fn test() {
    let result = Solution::number_of_ways("SSPPSPS".to_string());
    assert_eq!(result, 3);
}