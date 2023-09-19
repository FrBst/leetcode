struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pairs: Vec<(i32, i32)> = group_sizes.iter()
            .enumerate()
            .map(|(index, &value)| (index as i32, value))
            .collect();
        
        pairs.sort_by(|a, b| a.1.cmp(&b.1));
        
        let mut index = 0;
        let mut res = Vec::new();
        while index < pairs.len() {
            let group_size = pairs[index].1 as usize;
            res.push(pairs[index..index+group_size].iter()
                .map(|tuple| tuple.0)
                .collect());
            index += group_size;
        }

        res
    }
}