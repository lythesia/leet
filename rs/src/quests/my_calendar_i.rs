/**
 * [729] My Calendar I
 *
 * You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a double booking.
A double booking happens when two events have some non-empty intersection (i.e., some moment is common to both events.).
The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start <= x < end.
Implement the MyCalendar class:

	MyCalendar() Initializes the calendar object.
	boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a double booking. Otherwise, return false and do not add the event to the calendar.

 
Example 1:
Input
["MyCalendar", "book", "book", "book"]
[[], [10, 20], [15, 25], [20, 30]]
Output
[null, true, false, true]
Explanation
MyCalendar myCalendar = new MyCalendar();
myCalendar.book(10, 20); // return True
myCalendar.book(15, 25); // return False, It can not be booked because time 15 is already booked by another event.
myCalendar.book(20, 30); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
 
Constraints:

	0 <= start < end <= 109
	At most 1000 calls will be made to book.

 */
pub struct Solution {}

// submission codes start here

struct MyCalendar {
    vec: Vec<(i32,i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self {
            vec: vec![(-1,-1)],
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        match self.vec.binary_search_by_key(&start, |p| p.0) {
            Ok(_) => false,
            Err(pos) => {
                if pos == self.vec.len() {
                    if self.vec[pos - 1].1 <= start {
                        self.vec.push((start, end));
                        true
                    } else {
                        false
                    }
                } else if self.vec[pos-1].1 <= start && end <= self.vec[pos].0 {
                    self.vec.insert(pos, (start, end));
                    true
                } else {
                    false
                }
            }
        }
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
