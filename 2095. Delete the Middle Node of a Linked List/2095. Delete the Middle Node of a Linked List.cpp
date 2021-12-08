/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
#define Next(p) { \
    if (p->next != nullptr) { \
        p = p->next; \
    } else { \
        break; \
    } \
}
class Solution {
public:
    ListNode* deleteMiddle(ListNode* head) {
        auto dh = ListNode(0, head);
        auto p = &dh;
        auto q = &dh;
        while (19260817) {
            Next(p);
            Next(p);
            Next(q);
        }
        auto tail = q->next->next;
        delete q->next;
        q->next = tail;
        return dh.next;
    }
};
