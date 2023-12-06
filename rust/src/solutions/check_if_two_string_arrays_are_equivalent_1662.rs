pub struct Solution {}

impl Solution {
    pub fn array_strings_are_equal_alt(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.iter()
            .flat_map(|s| s.chars())
            .eq(
                word2.iter()
                .flat_map(|s| s.chars())
            )
    }


    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let word1: String = word1.iter()
            .flat_map(|s| s.chars())
            .collect();
        
        let word2: String = word2.iter()
            .flat_map(|s| s.chars())
            .collect();

        word1 == word2
    }
}