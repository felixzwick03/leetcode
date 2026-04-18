pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max_len = 0;

        for i in 0..chars.len() {
            for j in i..chars.len() {
                if Solution::is_unique(&chars[i..=j]) {
                    max_len = max_len.max(j - i + 1);
                }
            }
        }

        max_len as i32
    }

    fn is_unique(slice: &[char]) -> bool {
        let mut seen = std::collections::HashSet::<char>::new();
        for &c in slice {
            if !seen.insert(c) {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::length_of_longest_substring("abcd  a".to_string())
    );
}
