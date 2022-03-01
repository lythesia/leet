/**
 * [733] Flood Fill
 *
 * An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.
You are also given three integers sr, sc, and newColor. You should perform a flood fill on the image starting from the pixel image[sr][sc].
To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with newColor.
Return the modified image after performing the flood fill.
 
Example 1:
Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, newColor = 2
Output: [[2,2,2],[2,2,0],[2,0,1]]
Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.

Example 2:
Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, newColor = 2
Output: [[2,2,2],[2,2,2]]

 
Constraints:

	m == image.length
	n == image[i].length
	1 <= m, n <= 50
	0 <= image[i][j], newColor < 216
	0 <= sr < m
	0 <= sc < n

 */
pub struct Solution {}

// submission codes start here

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Solution {
    fn dfs(img: &mut Vec<Vec<i32>>,
        m: i32, n: i32,
        x: usize, y: usize,
        old: i32, new: i32) {
        let cur = img[x][y];
        if cur != old || cur == new { return; }
        else {
            img[x][y] = new;
            for (dx, dy) in DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m && ny >=0 && ny < n {
                    Self::dfs(img, m, n, nx as usize, ny as usize, old, new);
                }
            }
        }
    }

    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (m, n) = (image.len() as i32, image[0].len() as i32);
        let (sr, sc) = (sr as usize, sc as usize);
        let old_color = image[sr][sc];
        Self::dfs(&mut image, m, n, sr, sc, old_color, new_color);
        image
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec_vec![[2,2,2],[2,2,0],[2,0,1]],
        Solution::flood_fill(vec_vec![[1,1,1],[1,1,0],[1,0,1]], 1, 1, 2));

        assert_eq!(vec_vec![[2,2,2],[2,2,2]],
        Solution::flood_fill(vec_vec![[0,0,0],[0,0,0]], 0, 0, 2));
    }
}
