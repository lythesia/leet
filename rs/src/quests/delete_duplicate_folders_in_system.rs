/**
 * [1948] Delete Duplicate Folders in System
 *
 * Due to a bug, there are many duplicate folders in a file system. You are given a 2D array paths, where paths[i] is an array representing an absolute path to the ith folder in the file system.

	For example, ["one", "two", "three"] represents the path "/one/two/three".

Two folders (not necessarily on the same level) are identical if they contain the same non-empty set of identical subfolders and underlying subfolder structure. The folders do not need to be at the root level to be identical. If two or more folders are identical, then mark the folders as well as all their subfolders.

	For example, folders "/a" and "/b" in the file structure below are identical. They (as well as their subfolders) should all be marked:
	
		/a
		/a/x
		/a/x/y
		/a/z
		/b
		/b/x
		/b/x/y
		/b/z
	
	
	However, if the file structure also included the path "/b/w", then the folders "/a" and "/b" would not be identical. Note that "/a/x" and "/b/x" would still be considered identical even with the added folder.

Once all the identical folders and their subfolders have been marked, the file system will delete all of them. The file system only runs the deletion once, so any folders that become identical after the initial deletion are not deleted.
Return the 2D array ans containing the paths of the remaining folders after deleting all the marked folders. The paths may be returned in any order.
 
Example 1:
Input: paths = [["a"],["c"],["d"],["a","b"],["c","b"],["d","a"]]
Output: [["d"],["d","a"]]
Explanation: The file structure is as shown.
Folders "/a" and "/c" (and their subfolders) are marked for deletion because they both contain an empty
folder named "b".

Example 2:
Input: paths = [["a"],["c"],["a","b"],["c","b"],["a","b","x"],["a","b","x","y"],["w"],["w","y"]]
Output: [["c"],["c","b"],["a"],["a","b"]]
Explanation: The file structure is as shown. 
Folders "/a/b/x" and "/w" (and their subfolders) are marked for deletion because they both contain an empty folder named "y".
Note that folders "/a" and "/c" are identical after the deletion, but they are not deleted because they were not marked beforehand.

Example 3:
Input: paths = [["a","b"],["c","d"],["c"],["a"]]
Output: [["c"],["c","d"],["a"],["a","b"]]
Explanation: All folders are unique in the file system.
Note that the returned array can be in a different order as the order does not matter.

 
Constraints:

	1 <= paths.length <= 2 * 104
	1 <= paths[i].length <= 500
	1 <= paths[i][j].length <= 10
	1 <= sum(paths[i][j].length) <= 2 * 105
	path[i][j] consists of lowercase English letters.
	No two paths lead to the same folder.
	For any folder not at the root level, its parent folder will also be in the input.

 */
pub struct Solution {}

// submission codes start here

use std::{rc::Rc, cell::RefCell};
use std::collections::{HashMap, VecDeque};

type TireNode = Rc<RefCell<Tire>>;
struct Tire {
	name: String,
	rem: bool,
	subs: HashMap<String, TireNode>,
}
impl Tire {
	fn new(name: String) -> Self {
		Self {
			name,
			rem: false,
			subs: HashMap::new(),
		}
	}
}

impl Solution {
	fn dfs(root: &TireNode, h: &mut HashMap<String, TireNode>) -> String {
		let mut r = root.borrow_mut();

		let mut children = r.subs.iter().collect::<Vec<_>>();
		children.sort_by_key(|(s, _)| *s);
		let tree = children.iter().map(|(_, node)| {
			Self::dfs(node, h)
		}).collect::<Vec<_>>().join(",");

		if !tree.is_empty() {
			// serilize is important to be unique
			let ans = r.name.clone() + "(" + &tree + ")";
			if let Some(sub_tree) = h.get_mut(&tree) {
				r.rem = true;
				sub_tree.borrow_mut().rem = true;
			} else {
				h.insert(tree, root.clone());
			}
			ans
		} else {
			r.name.clone()
		}
	}

    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
		let dummy = Rc::new(RefCell::new(Tire::new("/".into())));
		// build tree
		for path in paths {
			let mut root = dummy.clone();
			for p in path {
				let node = Rc::new(RefCell::new(Tire::new(p.clone())));
				let x = root.borrow_mut().subs.entry(p).or_insert(node).clone();
				root = x.clone();
			}
		}
		// dfs rem
		let mut h = HashMap::new();
		Self::dfs(&dummy, &mut h);
		// ans
		let mut ans = Vec::new();
		let mut q = VecDeque::new();
		q.push_back((vec![], dummy));
		while let Some((path, node)) = q.pop_front() {
			let r = node.borrow();
			if !r.rem {
				for (sub_path, sub_node) in r.subs.iter().filter(|(_, v)| !v.borrow().rem) {
					let mut p = path.clone();
					p.push(sub_path.clone());
					if !sub_node.borrow().subs.is_empty() {
						q.push_back((p.clone(), sub_node.clone()));
					}
					ans.push(p);
				}
			}
		}
		ans
    }
}

// submission codes end


#[cfg(test)]
mod tests {
    use super::*;

	fn cmp(mut lhs: Vec<Vec<String>>, mut rhs: Vec<Vec<String>>) {
		lhs.sort();
		rhs.sort();
		assert_eq!(lhs, rhs);
	}

    #[test]
    fn test() {
		// cmp(vec_vec_string![["d"], ["d", "a"]],
		// 	Solution::delete_duplicate_folder(vec_vec_string![["a"],["c"],["d"],["a","b"],["c","b"],["d","a"]]));

		// cmp(vec_vec_string![["c"],["c","b"],["a"],["a","b"]],
		// 	Solution::delete_duplicate_folder(vec_vec_string![["a"],["c"],["a","b"],["c","b"],["a","b","x"],["a","b","x","y"],["w"],["w","y"]]));

		// cmp(vec_vec_string![["c"],["c","d"],["a"],["a","b"]],
		// 	Solution::delete_duplicate_folder(vec_vec_string![["a","b"],["c","d"],["c"],["a"]]));

		// cmp(vec_vec_string![["a"],["b"],["a","z"],["b","z"],["b","w"]],
		// Solution::delete_duplicate_folder(vec_vec_string![["a"],["a","x"],["a","x","y"],["a","z"],["b"],["b","x"],["b","x","y"],["b","z"],["b","w"]]));

		cmp(vec_vec_string![["a"],["b"],["a","c"],["a","d"],["b","e"],["b","c"],["a","d","e"],["b","c","d"]],
		Solution::delete_duplicate_folder(vec_vec_string![["a"],["a","c"],["a","d"],["a","d","e"],["b"],["b","e"],["b","c"],["b","c","d"],["f"],["f","h"],["f","h","i"],["f","j"],["g"],["g","j"],["g","h"],["g","h","i"]]));
    }
}
