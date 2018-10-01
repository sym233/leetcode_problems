struct VoteInfo {
    int candidate;
    int votes;
    VoteInfo(int candidate, int votes) : candidate(candidate), votes(votes) {}
};

struct DoubleLinkedListNode {
    DoubleLinkedListNode* prev;
    DoubleLinkedListNode* next;
    VoteInfo val;
    DoubleLinkedListNode(VoteInfo val) : prev(nullptr), next(nullptr), val(val) {} 
};

void insertBefore(DoubleLinkedListNode* node, DoubleLinkedListNode* newNode) {
    newNode->prev = node->prev;
    newNode->next = node;
    node->prev->next = newNode;
    node->prev = newNode;

}
void moveUp(DoubleLinkedListNode* node) {
    if (node->prev != nullptr) {
        node->prev->next = node->next;
        node->next->prev = node->prev;
        node->next = node->prev;
        node->prev = node->prev->prev;
        node->next->prev = node;
        node->prev->next = node;
    } else {
        cerr << "prev null error" << endl;
    }
}

class TopVotedCandidate {
public:
    DoubleLinkedListNode* doubleLinkedListHead;
    DoubleLinkedListNode* doubleLinkedListTail;
    vector<int> topVoted;
    vector<int> times;
    TopVotedCandidate(vector<int> persons, vector<int> times) : times(times) {
        topVoted.resize(0);
        
        // store cadidates and its vote in double linked list
        doubleLinkedListHead = new DoubleLinkedListNode(VoteInfo(-1, INT_MAX));
        doubleLinkedListTail = new DoubleLinkedListNode(VoteInfo(-1, INT_MIN));
        doubleLinkedListHead->next = doubleLinkedListTail;
        doubleLinkedListTail->prev = doubleLinkedListHead;
        // use map to find corresponding node faster
        unordered_map<int, DoubleLinkedListNode*> m;
        
        for (int person : persons) {
            unordered_map<int, DoubleLinkedListNode*>::iterator it = m.find(person);
            DoubleLinkedListNode* personNode;
            if (it != m.end()) {
                personNode = it->second;
                personNode->val.votes++;
            } else {
                personNode = new DoubleLinkedListNode(VoteInfo(person, 1));
                insertBefore(doubleLinkedListTail, personNode);
                m[person] = personNode;
            }
            
            // maintain list order
            while (personNode->val.votes >= personNode->prev->val.votes) {
                moveUp(personNode);
            }
            
            // get current top voted person
            topVoted.push_back(doubleLinkedListHead->next->val.candidate);
        }
    }
    
    int q(int t) {
        vector<int>::iterator it = upper_bound(times.begin(), times.end(), t);
        int i = it - times.begin();
        
        return topVoted[i - 1];
    }
};

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * TopVotedCandidate obj = new TopVotedCandidate(persons, times);
 * int param_1 = obj.q(t);
 */
 