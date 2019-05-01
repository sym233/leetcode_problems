/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    vector<int> nextLargerNodes(ListNode* head) {
        int l = 0;
        ListNode* rev = nullptr;
        while (head != nullptr) {
            ListNode* p = head;
            head = head->next;
            p->next = rev;
            rev = p;
            l++;
        }
        vector<int> res(l, 0);
        stack<int> s;
        ListNode* p = rev;
        while (l > 0) {
            if (s.empty()) {
                res[l - 1] = 0;
                s.push(p->val);
                l--;
                p = p->next;
            } else if (p->val < s.top()) {
                res[l - 1] = s.top();
                s.push(p->val);
                l--;
                p = p->next;
            } else {
                s.pop();
                continue;
            }
        }
        return res;
    }
};
