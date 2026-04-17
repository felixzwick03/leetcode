struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Solution::get_list_from_val(
            Solution::get_val_from_list(l1) + Solution::get_val_from_list(l2),
        );
    }

    pub fn get_val_from_list(list: Option<Box<ListNode>>) -> i32 {
        let mut string_val = "".to_string();
        let mut current_node = list.unwrap();
        let mut current_val = current_node.val;
        string_val.push_str(&current_val.to_string());
        while current_node.next != None {
            current_node = current_node.next.unwrap();
            current_val = current_node.val;
            string_val.push_str(&current_val.to_string());
        }
        return string_val
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }

    pub fn get_list_from_val(num: i32) -> Option<Box<ListNode>> {
        let mut result: ListNode;
        let char_vec: Vec<char> = num.to_string().chars().collect();
        let leave_node = ListNode::new(char_vec.get(0).unwrap().to_digit(10).unwrap() as i32);
        result = leave_node.clone();
        let mut previous_node = leave_node.clone();
        for i in 1..char_vec.len() {
            let current_value = char_vec.get(i).unwrap().to_digit(10).unwrap() as i32;
            result = ListNode {
                val: current_value,
                next: Some(Box::new(previous_node.clone())),
            };
            previous_node = result.clone();
        }
        return Some(Box::new(result.clone()));
    }
}

fn main() {
    // 1 -> 4 -> 7   =>  741
    let list_node_3 = ListNode::new(7);
    let list_node_2 = ListNode {
        val: 4,
        next: Some(Box::new(list_node_3)),
    };
    let list_node_1 = ListNode {
        val: 1,
        next: Some(Box::new(list_node_2)),
    };

    println!(
        "{:?}",
        Solution::get_val_from_list(Some(Box::new(list_node_1)))
    );

    println!("{:?}", Solution::get_list_from_val(741))
}
