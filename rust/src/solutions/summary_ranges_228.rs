pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = Vec::new();
    let mut start = 0;

    for i in 1..nums.len() + 1 {
        if nums.len() == i || nums[i - 1] != nums[i] - 1 {
            let start_value = nums[start];
            let end_value = nums[i - 1];

            res.push(if start == i - 1 {
                start_value.to_string()
            } else {
                format!("{}->{}", start_value, end_value)
            });
            start = i;
        }
    }

    res
}
