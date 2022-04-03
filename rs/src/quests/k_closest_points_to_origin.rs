/**
 * [973] K Closest Points to Origin
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k, return the k closest points to the origin (0, 0).
The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)2 + (y1 - y2)2).
You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).

Example 1:
Input: points = [[1,3],[-2,2]], k = 1
Output: [[-2,2]]
Explanation:
The distance between (1, 3) and the origin is sqrt(10).
The distance between (-2, 2) and the origin is sqrt(8).
Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].

Example 2:
Input: points = [[3,3],[5,-1],[-2,4]], k = 2
Output: [[3,3],[-2,4]]
Explanation: The answer [[-2,4],[3,3]] would also be accepted.


Constraints:

    1 <= k <= points.length <= 104
    -104 < xi, yi < 104

 */
pub struct Solution {}

// submission codes start here

fn d(x: &Vec<i32>) -> i32 {
    x[0].pow(2) + x[1].pow(2)
}
impl Solution {
    // partial sort
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = points.len();
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for x in points {
            if heap.len() < k as usize {
                heap.push((d(&x), x));
            } else {
                let xd = d(&x);
                match heap.peek() {
                    Some((topd, top)) if xd < *topd => {
                        heap.pop();
                        heap.push((xd, x));
                    }
                    _ => {},
                }
            }
        }
        heap.into_iter().map(|(_, x)| x).collect()
    }
}

// submission codes end

use rand::prelude::SliceRandom;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!(
            "{:?}",
            Solution::k_closest(vec_vec![[3, 3], [5, -1], [-2, 4]], 2)
        );
    }
}
