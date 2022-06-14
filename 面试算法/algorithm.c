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