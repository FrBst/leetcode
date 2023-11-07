pub struct Solution {}

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut eta = vec![0;dist.len()];

        for i in 0..dist.len() {
            eta[i] = (dist[i] as f32 / speed[i] as f32).ceil() as i32;
        }

        eta.sort();

        let mut current_time = 0;
        let mut dead_count = 0;
        for i in 0..eta.len() {
            if current_time >= eta[i] {
                return dead_count;
            } else {
                current_time += 1;
                dead_count += 1;
            }
        }

        dead_count
    }


    pub fn eliminate_maximum_rbird111(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut minutes: Vec<_> = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(a,b)| (a as f32 / b as f32).ceil() as i32)
            .collect();

        minutes.sort_unstable();
        minutes.into_iter()
            .zip(0..)
            .take_while(|&(n,i)| n > i)
            .count() as i32

    }
}