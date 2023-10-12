

pub struct Solution {}

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        
        let mut count = [0; 26];

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        for i in (0..s1.len()).step_by(2) {
            count[(s1[i] - b'a') as usize] += 1;
            count[(s2[i] - b'a') as usize] -= 1;
        }

        for num in count {
            if num != 0 {
                return false;
            }
        }

        for i in (1..s1.len()).step_by(2) {
            count[(s1[i] - b'a') as usize] += 1;
            count[(s2[i] - b'a') as usize] -= 1;
        }

        for num in count {
            if num != 0 {
                return false;
            }
        }

        true
    }
}