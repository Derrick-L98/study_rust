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
        // if (pHead == NULL || pHead->next == NULL)
        // {
        //     return pHead;
        // }
        // //递归调用
        // ListNode *ans = ReverseList(pHead->next);
        // //让当前结点的下一个结点的 next 指针指向当前节点
        // pHead->next->next = pHead;
        // //同时让当前结点的 next 指针指向NULL ，从而实现从链表尾部开始的局部反转
        // pHead->next = NULL;
        // return ans;

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
        ListNode *pre = NULL;  // pre指针指向已经反转好的链表的最后一个节点，初始化为null；
        ListNode *cur = pHead; // cur指针指向待反转链表的第一个节点，最开始第一个节点待反转，所以指向头指针；
        ListNode *next = NULL; // next指针指向待反转链表的第二个节点，目的是保存链表，因为cur改变指向后，后面的链表则失效了，所以需要保存
        while (cur)
        {
            next = cur->next; // next保存待反转指针的第一个节点
            cur->next = pre;  // cur的第一个节点保存已经反转好的链表的最后一个节点
            pre = cur;        //已经反转好的链表的最后一个节点保存待反转链表的第一个节点
            cur = next;       //待反转链表的第一个节点为反转指针的第一个节点
        }                     //使指针反转
        return pre;
    }
};

// 链表内指定区间反转

// 将一个节点数为 size 链表 m 位置到 n 位置之间的区间反转，要求时间复杂度 O(n)，空间复杂度 O(1)。 例如：
//     给出的链表为 1->2->3->4->5->NULL,
//     m = 2, n = 4m = 2, n = 4,
//     返回 1->4->3->2->5->NULL.

//             数据范围： 链表长度 0 < size
//             <= 10000 ，0 < m <= n <= size，链表中每个节点的值满足
//         | val | <= 1000 要求：时间复杂度 O(n) ，空间复杂度 O(n)
// 进阶：时间复杂度 O(n)，空间复杂度 O(1)

// {1,2,3,4,5},2,4
// {1,4,3,2,5}

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
    /**
     *
     * @param head ListNode类
     * @param m int整型
     * @param n int整型
     * @return ListNode类
     */
    ListNode *reverseBetween(ListNode *head, int m, int n)
    {
        // write code here
        // ｐ：指向ｍ前一个节点，ｑ指向第ｎ个节点。ｐ１，ｐ２用于翻转．
        if (head == NULL || head->next == NULL || m == n)
        {
            return head;
        }

        struct ListNode *p;
        struct ListNode *q;
        struct ListNode *l;
        l = (struct ListNode *)malloc(sizeof(struct ListNode)); //创建空白节点
        l->next = head;                                         // 空白节点指向头节点
        p = q = l;
        for (int i = 1; i < m; i++)
            p = p->next; // p指向m前一个节点
        for (int i = 1; i <= n; i++)
            q = q->next;               // q 指向第n个节点
        struct ListNode *p1 = p->next; // p1 是第m个节点
        struct ListNode *p2 = p1->next;
        p->next = q; // m前一个节点要指向q
        p1->next = q->next;
        while (p2 != q)
        { //反转节点
            p = p2->next;
            p2->next = p1;
            p1 = p2;
            p2 = p;
        }
        p2->next = p1;
        return l->next;

        // ListNode *node = new ListNode(-1); //创建一个新节点指向第一个节点。如果m=1，这时，m-1 节点是不存在的，这样就出现了问题，为了解决这个问题，我们在链表的头节点之前再加一个空节点来指向头节点，就可以了
        // node->next = head;
        // ListNode *cur = node;
        // for (int i = 0; i < m - 1; i++) //找到m前一个节点
        // {
        //     cur = cur->next;
        // }
        // ListNode *tmp = cur->next; //m节点
        // ListNode *ppre = tmp;
        // ListNode *pnode = tmp->next; // m节点下一个节点

        // ListNode *pnext = NULL;      //创建一个新节点指向m节点的下一个节点

        // for (int i = m + 1; i <= n; ++i) //找到n节点
        // {
        //     pnext = pnode->next;
        //     pnode->next = ppre;
        //     if (i == n)
        //     {
        //         tmp->next = pnext;
        //         if (m == 1)
        //             head = pnode;
        //         else
        //             cur->next = pnode;
        //     }
        //     ppre = pnode;
        //     pnode = pnext;
        // }
        // return head;
    }
};