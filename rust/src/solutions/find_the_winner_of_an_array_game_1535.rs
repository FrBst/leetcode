pub struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut win_count = 0;
        let mut winner = arr[0];

        for i in 1..arr.len() {
            if arr[i] > winner {
                win_count = 1;
                winner = arr[i];
            } else {
                win_count += 1;
            }

            if win_count == k {
                return winner;
            }
        }

        winner
    }
}