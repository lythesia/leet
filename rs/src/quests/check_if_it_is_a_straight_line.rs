/**
 * [1232] Check If It Is a Straight Line
 *
 * You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
 
 
Example 1:

Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
Output: true

Example 2:

Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
Output: false

 
Constraints:

	2 <= coordinates.length <= 1000
	coordinates[i].length == 2
	-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
	coordinates contains no duplicate point.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_straight_line(coor: Vec<Vec<i32>>) -> bool {
        let (dx, dy) = (coor[1][0]- coor[0][0], coor[1][1] - coor[0][1]);
        let mut last = coor[1].clone();
        coor[2..].iter().all(|p| {
            let (ndx, ndy) = (p[0] - last[0], p[1] - last[1]);
            last = p.clone();
            dx == 0 && ndx == 0 || dx != 0 && ndx*dy == ndy*dx
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
