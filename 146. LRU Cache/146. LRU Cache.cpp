class LRUCache {
private:
    template<typename T>
    struct BiLinkListNode {
        T val;
        BiLinkListNode<T>* prev;
        BiLinkListNode<T>* next;
        BiLinkListNode(T val) : val(val), prev(nullptr), next(nullptr) {}
        
        void deletePrev() {
            if (prev != nullptr) {
                BiLinkListNode<T>* toDel = prev;
                prev = toDel->prev;
                toDel->prev->next = this;
                toDel->prev = nullptr;
                toDel->next = nullptr;
                delete toDel;
            }
        }
        void insertNext(T val) {
            BiLinkListNode<T>* toIns = new BiLinkListNode<T>(val);
            toIns->prev = this;
            toIns->next = next;
            next->prev = toIns;
            next = toIns;
        }
    };
    
    int capa;
    int size;
    typedef pair<int, int> KVPair;
    typedef BiLinkListNode<KVPair> Node;
    typedef unordered_map<int, Node*> Map;
    Map m;
    Node* head;
    Node* tail;
    
public:
    LRUCache(int capacity) {
        capa = capacity;
        size = 0;
        head = new Node(make_pair<int, int>(-1, -1));
        tail = new Node(make_pair<int, int>(-1, -1));
        head->next = tail;
        tail->prev = head;
        m.clear();
    }
    
    int get(int key) {
        Map::iterator it = m.find(key);
        int value = -1;
        if (it != m.end()) {
            Node* node = it->second;
            value = node->val.second;
            // refresh cache
            node->next->deletePrev();
            head->insertNext(make_pair(key, value));
            it->second = head->next;
            
        }
        return value;
    }
    
    void put(int key, int value) {
        Map::iterator it = m.find(key);
        if (it != m.end()) {
            Node* node = it->second;
            node->next->deletePrev();
            head->insertNext(make_pair(key, value));
            it->second = head->next;
        } else {
            if (size < capa) {
                head->insertNext(make_pair(key, value));
                m[key] = head->next;
                size++;
            } else {
                // delete least used
                int keyToDel = tail->prev->val.first;
                m.erase(keyToDel);
                tail->deletePrev();
                
                // insert new
                head->insertNext(make_pair(key, value));
                m[key] = head->next;
            }
        }
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */
