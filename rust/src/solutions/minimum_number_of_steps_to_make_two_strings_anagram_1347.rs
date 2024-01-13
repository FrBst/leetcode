pub struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count: Vec<i32> = vec![0;26];
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            count[(s[i] - b'a') as usize] += 1; 
            count[(t[i] - b'a') as usize] -= 1; 
        }

        count.iter().filter(|&&x| x > 0).sum()
    }
}