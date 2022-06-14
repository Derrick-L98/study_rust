//反转链表

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }
}

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     * @param head ListNode类
     * @return ListNode类
     */
    pub fn ReverseList(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // write code here
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            //使用while let 时， 不可copy的cur必须被处理。
            cur = node.next; // 改变cur的指向，
            node.next = prev; // 把next指向前一个节点
            prev = Some(node); // 更新 prev
        }
        prev // 跳出循环时,prev是最终的头节点


        //方法2 迭代
        if(head==None){
            return None;
        }
        // 存放当前节点的next指针指向的节点
        let mut node_p1: Option<Box<ListNode>> = None;
        // 存放下一个节点
        let mut node_p2 = &head;
        while let Some(node) = node_p2{
            node_p1 = Some(Box::new(ListNode {
                    next: node_p1, 
                    val: node.val}));
            node_p2 = &node.next;
        }
        node_p1
    }
}
