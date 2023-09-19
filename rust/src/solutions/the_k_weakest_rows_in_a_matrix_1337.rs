use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        for (index, row) in mat.iter().enumerate() {
            let score: i32 = row.iter().sum();
            heap.push((score, index as i32));
            
            if (heap.len() > k as usize) {
                heap.pop();
            }
        }

        heap.into_sorted_vec().iter().map(|tuple| tuple.1).collect()
    }
}