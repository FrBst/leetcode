use std::{collections::HashMap, char};

pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut result: u8 = 0;

        for c in s.bytes().chain(t.bytes()) {
            result = result ^ c;
        }

        char::from(result)
    }
}