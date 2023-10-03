pub struct Solution {}

impl Solution {
    // The Rust Way
    pub fn winner_of_game(colors: String) -> bool {
        colors.as_bytes().windows(3).fold(0, |acc, w| {
            acc + match w {
                b"AAA"=> 1,
                b"BBB" => -1,
                _ => 0
            }
        }) > 0
    }

    // The Usual Way
    pub fn winner_of_game_alt(colors: String) -> bool {
        let mut count_a = 0;
        let mut count_b = 0;

        let colors = colors.as_bytes();

        let mut current: i32 = 0;
        for i in 0..colors.len() {
            current += 1;
            if colors[i] == b'A' && (i+1 == colors.len() || colors[i+1] == b'B') {
                count_a += std::cmp::max(current - 2, 0);
                current = 0;
            } else 
            if colors[i] == b'B' && (i+1 == colors.len() || colors[i+1] == b'A') {
                count_b += std::cmp::max(current - 2, 0);
                current = 0;
            }
        }

        count_a > count_b
    }
}