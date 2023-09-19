use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut rendered = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0,0 as usize)));

        let mut result = 0;

        while rendered.len() < points.len() {
            let Reverse((distance, index)) = queue.pop().expect("wtf");
            if rendered.contains(&index) {
                continue;
            }

            result += distance;
            rendered.insert(index);

            let p = &points[index];
            for (index, coords) in points.iter().enumerate() {
                if (rendered.contains(&index)) {
                    continue;
                }
                let distance = (&coords[0] - &p[0]).abs() + (&coords[1] - &p[1]).abs();
                queue.push(Reverse((distance, index)));
            }
        }

        result
    }
}