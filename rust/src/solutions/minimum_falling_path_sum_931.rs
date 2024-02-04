pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut matrix = matrix.to_vec();

        for i in 1..n {
            for j in 0..n {
                let mut min = matrix[i-1][j];
                if j != 0 {
                    min = min.min(matrix[i-1][j-1]);
                }
                if j != n - 1 {
                    min = min.min(matrix[i-1][j+1]);
                }

                matrix[i][j] += min;
            }
        }

        *matrix[n-1].iter().min().unwrap()
    }
}