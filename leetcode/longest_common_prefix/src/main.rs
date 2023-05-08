// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".



// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.



// Constraints:

//     1 <= strs.length <= 200
//     0 <= strs[i].length <= 200
//     strs[i] consists of only lowercase English letters.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut ret = strs[0].clone();
        for s in strs {
            while !s.starts_with(&ret) {
                ret.pop();
                if ret.is_empty() {
                    return String::new();
                }
            }
        }

        ret
    }
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test0() {
		assert_eq!("fl", Solution::longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]));
	}
	#[test]
	fn test1() {
		assert_eq!("", Solution::longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]));
	}
    #[test]
    fn test2() {
        assert_eq!("a", Solution::longest_common_prefix(vec![String::from("a")]));
    }
}

fn main() {
    println!("Pass test cases!");
}
