/**
 * [167] Two Sum II - Input Array Is Sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
The tests are generated such that there is exactly one solution. You may not use the same element twice.

Example 1:
Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].

Example 2:
Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].

Example 3:
Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].


Constraints:

	2 <= numbers.length <= 3 * 104
	-1000 <= numbers[i] <= 1000
	numbers is sorted in non-decreasing order.
	-1000 <= target <= 1000
	The tests are generated such that there is exactly one solution.

 */
pub struct Solution {}

// submission codes start here
/*
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a = std::iter::repeat(std::collections::VecDeque::new()).take(2001).collect::<Vec<_>>();

        for (i, x) in numbers.iter().enumerate() {
            let (i, x) = (i as i32, (x + 1000) as usize);
            a[x].push_back(i + 1);
        }

        for i in 0..2001 {
            if let Some(x) = a[i as usize].pop_front() {
                let j = (target - i + 2000);
                if j > 2000 || j < 0 { continue; }
                if let Some(y) = a[j as usize].pop_front() {
                    return vec![x, y];
                }
            }
        }

        vec![]
    }
}
*/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);

        while(i < j) {
            let s = numbers[i] + numbers[j];
            if s == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if s > target {
                j -= 1;
            } else {
                i += 1;
            }
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1,2], Solution::two_sum(vec![2,7,11,15], 9));
        assert_eq!(vec![1,3], Solution::two_sum(vec![2,3,4], 6));
        assert_eq!(vec![1,2], Solution::two_sum(vec![-1, 0], -1));
        assert_eq!(vec![1,2], Solution::two_sum(vec![0,0,3,4], 0));
        assert_eq!(vec![3,4], Solution::two_sum(vec![-1000,-1,0,1], 1));
    }
}
