// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution {}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        
        let (mut lo, mut hi) = (1, n);
        let mut index = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if self.isBadVersion(mid) {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
            index = mid;
        }

        index
    }

    fn isBadVersion(&self, version: i32) -> bool {
        true
    }
}