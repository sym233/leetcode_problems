/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */

int getLength(ListNode *node) {
    if (node == NULL) {
        return 0;
    } else {
        return getLength(node->next) + 1;
    }
}

struct NodeRecord {
    int length;
    ListNode* p;
    NodeRecord(int length, ListNode* p): length(length), p(p) {}
};

void heapInsert(vector<NodeRecord>& heap, NodeRecord node) {
    int i = heap.size();
    heap.push_back(node);
    while (i > 0 && heap[i].length < heap[(i - 1) / 2].length) {
        swap(heap[i], heap[(i - 1) / 2]);
        i = (i - 1) / 2; 
    }
    return;
}

NodeRecord heapPopTop(vector<NodeRecord>& heap) {
    NodeRecord top = heap[0];
    
    heap[0] = heap.back();
    heap.pop_back();
    int l = heap.size();
    
    int i = 0;
    
    while (2 * i + 1 < l) {
        int j = 2 * i + 1;
        if (heap[i].length > heap[j].length) {
            if (j + 1 < l && heap[j + 1].length < heap[j].length) {
                swap(heap[i], heap[j + 1]);
                i = j + 1;
            } else {
                swap(heap[i], heap[j]);
                i = j;
            }
        } else if (j + 1 < l && heap[i].length > heap[j + 1].length) {
            swap(heap[i], heap[j + 1]);
            i = j + 1;
        } else {
            break;
        }
    }
    return top;
}


ListNode* pullNextNode(ListNode* node) {
    if (node != NULL && node->next != NULL) {
        ListNode* t = node->next;
        node->next = node->next->next;
        t->next = NULL;
        return t;
    } else {
        return NULL;
    }
}

void merge2List(ListNode* list1, ListNode*& list2) {
    // list2 becomes empty and all its nodes move to list1
    
    ListNode* p1 = list1;
    
    while (list2->next != NULL) {
        if (p1->next != NULL) {
            if (list2->next->val < p1->next->val) {
                ListNode* t = pullNextNode(list2);
                t->next = p1->next;
                p1->next = t;
                p1 = p1->next;
            } else {
                p1 = p1->next;
            }
        } else {
            p1->next = list2->next;
            list2->next = NULL;
            break;
        }
    }
	
	// delete head node
    delete list2;
    list2 = NULL;
    
    
    return;
}


class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        int length = lists.size();
        
        if (length == 0) {
            return NULL;
        }
        
        vector<NodeRecord> heap;
        
        
        for (int i = 0; i < length; i++) {
            
            int l = getLength(lists[i]);
            
            // add head node
            ListNode *headNode = new ListNode(0);
            headNode->next = lists[i];
            
            NodeRecord n(l, headNode);
            heapInsert(heap, n);
            
            lists[i] = NULL;
        }
        
        for (int i = 0; i < length - 1; i++) {
            NodeRecord n1 = heapPopTop(heap);
            NodeRecord n2 = heapPopTop(heap);
            merge2List(n1.p, n2.p);
            n1.length += n2.length;
            heapInsert(heap, n1);
            
        }
        
        NodeRecord top = heapPopTop(heap);
        
        return (top.p)->next;
    }
};
