
pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![
            Self::search(&nums, target, true),
            Self::search(&nums, target, false)
        ]
    }

    fn search(nums: &Vec<i32>, target: i32, is_left: bool) -> i32 {

        let (mut lo, mut hi) = (0, nums.len() as i32 - 1);
        let mut mid = -1;

        while lo <= hi {
            mid = lo + (hi - lo) / 2;

            if nums[mid as usize] == target {
                if is_left { hi = mid - 1 } else { lo = mid + 1 };
                continue;
            }

            if nums[mid as usize] > target {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        mid
    }
}