/**
 * [1886] Determine Whether Matrix Can Be Obtained By Rotation
 *
 * Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.
 
Example 1:
Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
Output: true
Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.

Example 2:
Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
Output: false
Explanation: It is impossible to make mat equal to target by rotating mat.

Example 3:
Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
Output: true
Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.

 
Constraints:

	n == mat.length == target.length
	n == mat[i].length == target[i].length
	1 <= n <= 10
	mat[i][j] and target[i][j] are either 0 or 1.

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len() - 1;
        let mut rotations = [true; 4]; // rotate i*90 degrees

        mat.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &val)| {
                rotations
                    .iter_mut()
                    .enumerate()
                    .filter(|(_, flag)| **flag)
                    .for_each(|(rot, flag)| {
                        *flag = match rot {
                            0 => val == target[i][j],
                            1 => val == target[j][n - i],
                            2 => val == target[n - i][n - j],
                            _ => val == target[n - j][i],
                        }
                    });
            })
        });
        rotations.iter().any(|&x| x)
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
