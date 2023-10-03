struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {

        let window_len = s.len();

        // Calculate hash for 's'
        let mut s_hash: u32 = 0;
        for char in s.as_bytes() {
            s_hash += Self::hash(char);
        }
        let s_hash = s_hash;
        
        // Calculate hash for beginning of 't' with one less char than 's'
        let mut window_hash: u32 = 0;
        for char in t[0..window_len-1].as_bytes() {
            window_hash += Self::hash(char);
        }

        // Slide
        for (index, char) in t[window_len..].as_bytes().iter().enumerate() {
            window_hash += Self::hash(char);
            let candidate = &t[index-2*window_len+1..index-window_len+1];
            
            if window_hash == s_hash {
                if candidate.eq(s.as_str()) {
                    return true;
                }
            }
        }

        false

    }

    fn hash(char: &u8) -> u32 {
        31 * (*char as u32) + 7
    }
}