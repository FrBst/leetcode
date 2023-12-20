pub struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = Vec::new();

        let last = target[target.len()-1];
        let mut target_ptr = 0;
        for i in 1..=n {
            ans.push(String::from("Push"));
            if i == last {
                break;
            }
            if target[target_ptr] == i {
                target_ptr += 1;
            } else {
                ans.push(String::from("Pop"));
            }
        }

        ans
    }
}