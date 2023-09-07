use std::cmp;

pub struct Solution {}

impl Solution {

    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map = [MaxPair::new(); 10];
        for x in nums {
            map[Self::max_digit(x) as usize].push(x);
        }
    
        let mut max = -1;
        for digit in 0..10 {
            max = cmp::max(map[digit].get_sum(), max);
        }
        max
    }
    
    pub fn max_digit(num: i32) -> i32 {
        let mut num = num;
        let mut max = 0;
    
        while num > 0 {
            let digit = num % 10;
            num = num / 10;
            max = cmp::max(digit, max);
        }
    
        max
    }
}
    
#[derive(Copy, Clone)]
struct MaxPair {
    lesser: i32,
    greater: i32
}

impl MaxPair {
    pub fn new() -> MaxPair {
        MaxPair {
            lesser: -1,
            greater: -1
        }
    }

    pub fn clone(self) -> MaxPair {
        MaxPair {
            lesser: self.lesser,
            greater: self.greater
        }
    }

    pub fn push(&mut self, x: i32) {
        if x > self.greater {
            self.lesser = self.greater;
            self.greater = x;
        } else
        if x > self.lesser {
            self.lesser = x;
        }
    }

    pub fn get_sum(self) -> i32 {
        match self.lesser {
            -1 => -1,
            _ => self.lesser + self.greater
        }
    }
}