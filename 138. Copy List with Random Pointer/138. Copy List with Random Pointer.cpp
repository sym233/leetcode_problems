/**
 * Definition for singly-linked list with a random pointer.
 * struct RandomListNode {
 *     int label;
 *     RandomListNode *next, *random;
 *     RandomListNode(int x) : label(x), next(NULL), random(NULL) {}
 * };
 */
class Solution {
public:
    RandomListNode *copyRandomList(RandomListNode *head) {
        unordered_map<int, RandomListNode*> m;
        
        if (head == nullptr) {
            return nullptr;
        }
        
        RandomListNode *res;
        RandomListNode *p;
        RandomListNode *q;
        
        res = new RandomListNode(head->label);
        m[res->label] = res;
        
        p = head;
        q = res;
        for (; p->next != nullptr; p = p->next) {
            q->next = new RandomListNode(p->next->label);
            m[q->next->label] = q->next;
            q = q->next;
        }
        
        p = head;
        q = res;
        for (; p != nullptr; p = p->next) {
            if (p->random != nullptr) {
                int label = p->random->label;
                q->random = m[label];
            }
            q = q->next;
        }
        return res;
    }
};
