use std::collections::VecDeque;


struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        let mut n = senate.len();

        for (index, faction) in senate.as_bytes().iter().enumerate() {
            if *faction == b'R' {
                radiant.push_front(index)
            } else {
                dire.push_front(index)
            }
        }

        while !radiant.is_empty() && !dire.is_empty() {
            let next_rad = radiant.pop_back().unwrap();
            let next_dir = dire.pop_back().unwrap();

            n += 1;
            if (next_rad > next_dir) {
                dire.push_front(n);
            } else {
                radiant.push_front(n);
            }
        }

        String::from(if radiant.is_empty() { "Dire" } else { "Radiant" })
    }
}