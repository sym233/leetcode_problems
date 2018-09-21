/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
ListNode* pullNextNode(ListNode* node) {
    if (node->next != NULL) {
        ListNode* nextNode = node->next;
        node->next = node->next->next;
        nextNode->next = NULL;
        return nextNode;
    } else {
        return NULL;
    }
}


class Solution {
public:
    ListNode* partition(ListNode* head, int x) {
        if (head == NULL) {
            return NULL;
        }
        
        ListNode* greaterNodesHead = new ListNode(0);
        ListNode* greaterNodesTail = greaterNodesHead;
        
        ListNode* listHead = new ListNode(0);
        listHead->next = head;
        ListNode* p = listHead;
        
        while (p->next != NULL) {
            if (p->next->val >= x) {
                ListNode* t = pullNextNode(p);
                greaterNodesTail->next = t;
                greaterNodesTail = greaterNodesTail->next;
            } else {
                p = p->next;
            }
        }
        
        p->next = greaterNodesHead->next;
        
        ListNode* t = listHead->next;
        
        delete greaterNodesHead;
        delete listHead;
        return t;
    }
};
