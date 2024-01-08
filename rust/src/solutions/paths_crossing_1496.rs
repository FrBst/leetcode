use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::new();
        visited.insert((0,0));

        let mut current = (0,0);
        for c in path.as_bytes().iter() {
            match c {
                b'N' => current.1 += 1,
                b'S' => current.1 -= 1,
                b'E' => current.0 += 1,
                b'W' => current.0 -= 1,
                _ => ()
            }

            match visited.get(&current) {
                None => visited.insert(current),
                Some(_) => return true
            };
        }

        false
    }
}