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
class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        auto dh = ListNode(0);
        dh.next = head;
        auto i = &dh;
        while (i && i->next) {
            if (i->next->next && i->next->val == i->next->next->val) {
                while (i->next->next && i->next->val == i->next->next->val) {
                    auto j = i->next->next;
                    i->next->next = i->next->next->next;
                    delete j;
                }
                auto j = i->next;
                i->next = i->next->next;
                delete j;
            } else {
                i = i->next;
            }
        }
        return dh.next;
    }
};
