pub struct Solution {}

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let num = num.as_bytes();
        for i in (0..num.len()).rev() {
            if num[i] % 2 == 1 {
                return String::from(std::str::from_utf8(&num[0..=i]).unwrap());
            }
        }

        "".to_string()
    }
}