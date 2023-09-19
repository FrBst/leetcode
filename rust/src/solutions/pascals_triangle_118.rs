
struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        let mut res = Vec::new();
        res.push(vec![1]);

        for lvl in 1..num_rows {
            let prev_layer = &res[lvl-1];
            let mut new_layer = vec![0; prev_layer.len() + 1];
            for column in 0..prev_layer.len() {
                new_layer[column] += prev_layer[column];
                new_layer[column+1] += prev_layer[column];
            }

            res.push(new_layer);
        }
        
        res
    }
}