struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solution: Vec<i32> = Vec::new();
        'outer: for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if (nums.get(i).unwrap() + nums.get(j).unwrap() == target) {
                    solution.push(i as i32);
                    solution.push(j as i32);
                    break 'outer;
                }
            }
        }
        return solution;
    }
}

fn main() {
    let nums: Vec<i32> = Vec::from([1, 5, -2, 7]);
    let target: i32 = 6;
    println!("{:?}", Solution::two_sum(nums, target));
}
