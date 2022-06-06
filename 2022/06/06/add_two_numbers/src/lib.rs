// https://leetcode.com/problems/add-two-numbers/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode {
#[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution {}
impl Solution {
    pub fn add_two_numbers(list_1: Option<Box<ListNode>>, list_2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list_1_values = Self::list_to_vector(list_1);
        let mut list_2_values = Self::list_to_vector(list_2);
        let mut total_sum = vec![];
        {
            let mut carry = 0;
            let mut index = 0;
            loop {
                let mut list_1_value = 0;
                if index < list_1_values.len() {
                    list_1_value = list_1_values.get(index).unwrap().clone();
                }
                let mut list_2_value = 0;
                if index < list_2_values.len() {
                    list_2_value = list_2_values.get(index).unwrap().clone();
                }
                total_sum.push((list_1_value + list_2_value + carry) % 10);
                carry = (list_1_value + list_2_value + carry) / 10;
                index += 1;
                if (index >= list_1_values.len()) && (index >= list_2_values.len()) {
                    break;
                }
            }
            if carry > 0 {
                total_sum.push(1);
            }
        }
        Self::vector_to_list(total_sum)
    }
    // [Example] make_list([2,4,3]) -> ListNode(2, ListNode(4, ListNode(3, None))), which is equal to 342 in a decimal expression.
    pub fn vector_to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for number in vector.iter().rev() {
            let current = Some(Box::new(ListNode {
                val: *number,
                next: std::mem::replace(&mut head, None)
            }));
            head = current;
        }
        head
    }
    // Reverse of vector_to_list()
    pub fn list_to_vector(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut current = list;
        while current != None {
            let reference = std::mem::replace(&mut current, None).unwrap();
            let node = *reference;
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::ListNode;

    #[test]
    fn example_1() {
        let list_1 = Solution::vector_to_list(vec![2, 4, 3]);
        let list_2 = Solution::vector_to_list(vec![5, 6, 4]);
        let answer = Solution::vector_to_list(vec![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(list_1, list_2), answer);
    }

    #[test]
    fn example_2() {
        let list_1 = Solution::vector_to_list(vec![0]);
        let list_2 = Solution::vector_to_list(vec![0]);
        let answer = Solution::vector_to_list(vec![0]);
        assert_eq!(Solution::add_two_numbers(list_1, list_2), answer);
    }

    #[test]
    fn example_3() {
        let list_1 = Solution::vector_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let list_2 = Solution::vector_to_list(vec![9, 9, 9, 9]);
        let answer = Solution::vector_to_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(list_1, list_2), answer);
    }
}