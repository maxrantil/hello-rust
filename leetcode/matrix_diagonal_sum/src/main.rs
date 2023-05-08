// Input: mat = [[1,2,3],
//               [4,5,6],
//               [7,8,9]]
// Output: 25
// Explanation: Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
// Notice that element mat[1][1] = 5 is counted only once.

// Example 2:

// Input: mat = [[1,1,1,1],
//               [1,1,1,1],
//               [1,1,1,1],
//               [1,1,1,1]]
// Output: 8

// Example 3:

// Input: mat = [[5]]
// Output: 5



// Constraints:

//     n == mat.length == mat[i].length
//     1 <= n <= 100
//     1 <= mat[i][j] <= 100

struct Solution;

impl Solution {
	pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
		let mut sum = 0;
		let mut i = 0;
		let mut j = 0;
        let mut k = mat.len() - 1;
	  	while i < mat.len() {
            if k != i {
                sum += mat[i][k];
            }
            sum += mat[i][j];
			i += 1;
			j += 1;
			if k > 0 {
				k -= 1;
			}
		}
		sum
    }
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test() {
		assert_eq!(25, Solution::diagonal_sum(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]));
	}
	#[test]
	fn test2() {
		assert_eq!(8, Solution::diagonal_sum(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]));
	}
}

fn main() {
  	println!("Pass test cases!");
}