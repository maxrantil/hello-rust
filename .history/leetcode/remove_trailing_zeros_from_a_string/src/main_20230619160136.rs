struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
      let mut num2;
      num.trim_end_matches('0').to_string();
      println!("{}", num2);
      num2
    }
}

fn main() {
  Solution::remove_trailing_zeros("00000012300000".to_string());
}
