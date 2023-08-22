use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn from(ar: Vec<i32>) -> Option<Box<ListNode>> {
        ar.iter().rev().fold(None, |acc, i| {
            Option::Some(Box::new(ListNode { val: *i, next: acc }))
        })
    }

    fn to_vec(self) -> Vec<i32> {
        let mut res = Vec::new();
        let mut ptr = Some(Box::new(self));
        while let Some(node) = ptr {
            res.push(node.val);
            ptr = node.next;
        }
        return res;
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        while let Some(ref mut head) = curr {
            while let Some(ref mut node) = &mut head.next {
                if head.val != node.val {
                    break;
                } else {
                    head.next = node.next.take();
                }
            }
            curr = &mut head.next;
        }

        head
    }
}

#[cfg(test)]
mod test {
    use crate::{remove_duplicates_list::ListNode, Solution};

    #[test]
    fn remove_duplicates_list() {
        for (input, expected) in [([1, 2, 2, 3, 4, 5, 5, 6], [1, 2, 3, 4, 5, 6])] {
            let list = ListNode::from(Vec::from(input));
            let res = Solution::delete_duplicates(list);
            assert_eq!((*res.unwrap()).to_vec(), Vec::from(expected))
        }
    }
}
