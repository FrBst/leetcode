pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        
        let (mut candidate1, mut candidate2) = (0,0);
        let (mut count1, mut count2) = (0,0);

        for num in &nums {
            if *num == candidate1 {
                count1 += 1;
            }
            else if *num == candidate2 {
                count2 += 1;
            }
            else if count1 == 0 {
                candidate1 = *num;
                count1 = 1;
            }
            else if count2 == 0 {
                candidate2 = *num;
                count2 = 1;
            }
            else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        count1 = 0;
        count2 = 0;

        for num in &nums {
            if *num == candidate1 {
                count1 += 1;
            } else
            if *num == candidate2 {
                count2 += 1;
            }
        }

        let mut res = Vec::with_capacity(2);
        if count1 * 3 > nums.len() {
            res.push(candidate1);
        }
        if count2 * 3 > nums.len() {
            res.push(candidate2);
        }
        res
    }
}