pub struct Solution {}

impl Solution {
    // pub fn decode_at_index(s: String, k: i32) -> String {
    //     let mut total_length = 0;
    //     let last_index;
    //     for (index, c) in s.as_bytes().iter().enumerate() {
    //         if *c >= b'0' && *c <= b'9' {
    //             total_length *= (c - b'0') as i32;
    //         } else {
    //             total_length += 1;
    //         }
    //         if (total_length > k) {
    //             last_index = index;
    //             break;
    //         }
    //     }

    //     for i in last_index..0 {
    //         if (i == k) {
    //             return  &s[k..k];
    //         }
    //         if (s[i] >= b'0' && s[i] <= b'9') {
    //             total_length /= (s[i] - b'0');
    //         } else {
    //             total_length -= 1;
    //         }
    //     }

    //     String::new()
    // }
}