/**
 * [739] Daily Temperatures
 *
 * Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
 
Example 1:
Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]
Example 2:
Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]
Example 3:
Input: temperatures = [30,60,90]
Output: [1,1,0]
 
Constraints:

	1 <= temperatures.length <= 105
	30 <= temperatures[i] <= 100

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut mono = vec![];
        let mut ans = vec![0; t.len()];
        for (i, x) in t.into_iter().enumerate() {
            while let Some((j, top)) = mono.pop() {
                if top < x {
                    ans[j] = (i - j) as i32;
                } else {
                    mono.push((j, top)); // restore
                    break;
                }
            }
            mono.push((i, x));
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
        assert_eq!(vec![1,1,4,2,1,1,0,0], Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]));
    }
}
