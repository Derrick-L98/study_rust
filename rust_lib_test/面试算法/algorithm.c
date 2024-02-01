#include <stdio.h>
//反转链表

struct ListNode
{
    int val;
    struct ListNode *next;
};

//  C语言声明定义全局变量请加上static，防止重复定义

/**
 *
 * @param pHead ListNode类
 * @return ListNode类
 */
struct ListNode *ReverseList(struct ListNode *pHead)
{
    // write code here
    if (NULL == pHead)
        return NULL;
    if (NULL == pHead->next)
        return pHead;
    struct ListNode *l = pHead;
    struct ListNode *r = pHead->next;

    pHead->next = NULL;
    while (r)
    {
        struct ListNode *pTmp = r->next;
        r->next = l;
        l = r;
        r = pTmp;
    }
    return l;
}

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