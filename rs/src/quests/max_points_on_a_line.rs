/**
 * [149] Max Points on a Line
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
 
Example 1:
Input: points = [[1,1],[2,2],[3,3]]
Output: 3

Example 2:
Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
Output: 4

 
Constraints:

	1 <= points.length <= 300
	points[i].length == 2
	-104 <= xi, yi <= 104
	All the points are unique.

 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x%y)
    }
}
impl Solution {
    pub fn max_points(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut ans = 0;
        for i in 0..n {
            let mut mmax = 0;
            let mut h = HashMap::new();
            let pi = (points[i][0],points[i][1]);
            for j in i+1..n {
                let pj = (points[j][0], points[j][1]);
                let (dx, dy) = (pi.0 - pj.0, pi.1 - pj.1);
                if dx == 0 {
                    let cnt = h.entry((0, 1)).or_insert(0);
                    *cnt += 1;
                    mmax = mmax.max(*cnt);
                } else {
                    let g = gcd(dx, dy);
                    let cnt = h.entry((dx/g, dy/g)).or_insert(0);
                    *cnt += 1;
                    mmax = mmax.max(*cnt);
                }
            }
            ans = ans.max(mmax + 1);
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", Solution::max_points(vec_vec![[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]));
    }
}
