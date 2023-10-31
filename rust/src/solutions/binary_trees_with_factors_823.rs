use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {

        let mut res = 0;
        
        let mut arr = arr;
        arr.sort();

        let mut dp: HashMap<i32, i32> = HashMap::new();
        for &num in &arr {
            dp.insert(num, 1);
        }

        for &num in &arr {
            for &factor in &arr {
                if num == factor {
                    break;
                }
                let second_factor = num / factor;
                if num % factor == 0 && dp.contains_key(&second_factor) {
                    *dp.get_mut(&num).unwrap() += *dp.get(&factor).unwrap() + *dp.get(&second_factor).unwrap();
                }
            }
        }

        15
    }
}