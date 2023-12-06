pub struct Solution {}

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut dp = vec![1; 10];

        let MOD = 1_000_000_007;

        let paths = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![0, 3, 9],
            vec![],
            vec![0, 1, 7],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
            vec![4, 6]
        ];

        for i in 1..n {
            let mut temp = vec![0; 10];
            for j in 0..10 {
                temp[j] = paths[j].iter().map(|x| dp[*x]).sum::<u32>() % MOD;
            }
            dp = temp;
        }

        (0..10).fold(0, |acc, num| (acc + dp[num]) % MOD) as i32
    }
}