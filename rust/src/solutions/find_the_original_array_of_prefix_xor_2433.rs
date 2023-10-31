pub struct Solution {}

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {

        let mut arr = vec![0;pref.len()];
        arr[0] = pref[0];
        
        for i in 1..pref.len() {
            arr[i] = pref[i-1] ^ pref[i];
        }

        arr
    }
}