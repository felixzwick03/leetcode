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
        return Solution::get_list_from_string(Solution::add_strings(
            Solution::get_val_from_list(l1),
            Solution::get_val_from_list(l2),
        ));
    }

    pub fn get_val_from_list(list: Option<Box<ListNode>>) -> String {
        let mut string_val = "".to_string();
        let mut current_node = list.unwrap();
        let mut current_val = current_node.val;
        string_val.push_str(&current_val.to_string());
        while current_node.next != None {
            current_node = current_node.next.unwrap();
            current_val = current_node.val;
            string_val.push_str(&current_val.to_string());
        }
        return string_val.chars().rev().collect::<String>();
    }

    pub fn add_strings(num1: String, num2: String) -> String {
        let mut result = String::new();
        let mut carry = 0;
        let mut i = num1.len() as i32 - 1;
        let mut j = num2.len() as i32 - 1;

        while i >= 0 || j >= 0 || carry > 0 {
            let digit1 = if i >= 0 {
                num1.as_bytes()[i as usize] - b'0'
            } else {
                0
            };
            let digit2 = if j >= 0 {
                num2.as_bytes()[j as usize] - b'0'
            } else {
                0
            };

            let sum = digit1 as i32 + digit2 as i32 + carry;
            carry = sum / 10;
            let digit = (sum % 10) as u8 + b'0';
            result.push(digit as char);

            i -= 1;
            j -= 1;
        }

        result.chars().rev().collect()
    }

    pub fn get_list_from_string(str: String) -> Option<Box<ListNode>> {
        let mut result: ListNode;
        let char_vec: Vec<char> = str.chars().collect();
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
    /* let list_node_3 = ListNode::new(7);
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

    println!("{:?}", Solution::get_list_from_val(741))*/
    println!(
        "{:?}",
        Solution::add_strings("123".to_string(), "321".to_string())
    );
}
