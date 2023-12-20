pub struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let imax = img.len();
        let jmax = img[0].len();
        let mut res = vec![vec![0; jmax]; imax];

        for i in 0..imax as i32 {
            for j in 0..jmax as i32 {

                let mut sum = 0;
                let mut count = 0;

                for x in 0.max(i-1)..=(i+1) {
                    for y in 0.max(j-1)..=(j+1) {
                        if (x as usize) < imax && (y as usize) < jmax {
                            sum += img[x as usize][y as usize];
                            count += 1;
                        }
                    }
                }

                res[i as usize][j as usize] = if sum == 0 { 0 } else { sum / count };
            }
        }

        res
    }
}