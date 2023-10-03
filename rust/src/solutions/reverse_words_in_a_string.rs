

pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ")
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn reverse_words_declarative(s: String) -> String {
        let words: Vec<_> = s.split(" ").collect();
        let mut res = String::with_capacity(s.len() + 1);

        for word in words {
            for character in word.chars().rev() {
                res.push(character);
            }
            res.push(' ');
        }

        res.pop();
        res
    }
}