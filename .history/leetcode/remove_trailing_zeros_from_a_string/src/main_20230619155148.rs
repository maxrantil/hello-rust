struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
      println!("Hello, world!");
      let mut num2;
      num2 = num.chars().rev().collect::<String>();
      println!("{}", num2);
      num2 = num2.trim_start_matches('0').to_string;
      println!("{}", num2);
      num2 = num.chars().rev().collect::<String>();
      num
    }
}

fn main() {
  Solution::remove_trailing_zeros("00000012300000".to_string());
}
