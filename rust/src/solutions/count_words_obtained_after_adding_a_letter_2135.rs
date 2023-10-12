use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {

        let mut count = 0;
        let mut set = HashSet::with_capacity(start_words.len());

        for word in start_words {
            set.insert(Self::get_bits(word));
        }

        for word in target_words {
            let bits = Self::get_bits(word);

            for i in 0..26 {
                let bits_minus_one = bits & !(1 << i);
                if bits != bits_minus_one && set.contains(&bits_minus_one) {
                    count += 1;
                    break;
                }
            }
        }

        count    
    }

    fn get_bits(word: String) -> i32 {
        let mut bits = 0;
        for c in word.bytes() {
            bits += 1 << (c - b'a');
        }

        bits
    }
}