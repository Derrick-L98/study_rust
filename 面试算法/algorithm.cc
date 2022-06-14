#include <iostream>
//反转链表

// 解法二：递归
// 使用递归函数，一直递归到链表的最后一个结点，该结点就是反转后的头结点，记作 ans
// 此后，每次函数在返回的过程中，让当前结点的下一个结点的 next 指针指向当前节点。
// 同时让当前结点的 next 指针指向NULL ，从而实现从链表尾部开始的局部反转
// 当递归函数全部出栈后，链表反转完成。

// 复杂度分析：
// 时间复杂度：O(N)，其中 N 是链表的长度。需要对链表的每个节点进行反转操作。
// 空间复杂度：O(N)，其中 N 是链表的长度。空间复杂度主要取决于递归调用的栈空间，最多为 N 层
struct ListNode
{
    int val;
    struct ListNode *next;
    ListNode(int x) : val(x), next(NULL)
    {
    }
};
class Solution
{
public:
    ListNode *ReverseList(ListNode *pHead)
    {
        //特判：注意不要漏掉pHead->next==NULL的情况
        if (pHead == NULL || pHead->next == NULL)
        {
            return pHead;
        }
        //递归调用
        ListNode *ans = ReverseList(pHead->next);
        //让当前结点的下一个结点的 next 指针指向当前节点
        pHead->next->next = pHead;
        //同时让当前结点的 next 指针指向NULL ，从而实现从链表尾部开始的局部反转
        pHead->next = NULL;
        return ans;

        //方法2
        // ListNode *pre = nullptr;
        // while (pHead)
        // {                              // 判断链表是否已经到结尾
        //     ListNode *t = pHead->next; // 保留下一个即将翻转的表头
        //     pHead->next = pre;         // 将现在要翻转的节点的后继指向前一个节点 pre
        //     pre = pHead;               // 现在的 P 成为下一个前继节点
        //     pHead = t;                 // p 成为下一个要翻转的节点
        // }
        // return pre;

        //方法3
        // ListNode *pre = NULL;  // pre指针指向已经反转好的链表的最后一个节点，初始化为null；
        // ListNode *cur = pHead; // cur指针指向待反转链表的第一个节点，最开始第一个节点待反转，所以指向头指针；
        // ListNode *next = NULL; // next指针指向待反转链表的第二个节点，目的是保存链表，因为cur改变指向后，后面的链表则失效了，所以需要保存
        // while (cur)
        // {
        //     next = cur->next; // next保存待反转指针的第一个节点
        //     cur->next = pre;  // cur的第一个节点保存已经反转好的链表的最后一个节点
        //     pre = cur;        //已经反转好的链表的最后一个节点保存待反转链表的第一个节点
        //     cur = next;       //待反转链表的第一个节点为反转指针的第一个节点
        // }                     //使指针反转
        // return pre;
    }
};