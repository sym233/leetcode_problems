class LRUCache {
private:
    typedef pair<int, int> KVPair;
    typedef list<KVPair> List;
    typedef unordered_map<int, List::iterator> Map;
    Map m;
    List l;
    int capacity;
    
public:
    LRUCache(int capacity) : capacity(capacity) {
        m.clear();
        l.clear();
    }
    
    int get(int key) {
        Map::iterator it = m.find(key);
        int value = -1;
        if (it != m.end()) {
            List::iterator node = it->second;
            value = node->second;
            // refresh cache
            l.erase(node);
            l.push_front(make_pair(key, value));
            it->second = l.begin();
        }
        
        return value;
    }
    
    void put(int key, int value) {
        Map::iterator it = m.find(key);
        if (it != m.end()) {
            List::iterator node = it->second;
            l.erase(node);
            l.push_front(make_pair(key, value));
            it->second = l.begin();
        } else {
            if (l.size() < capacity) {
                l.push_front(make_pair(key, value));
                m[key] = l.begin();
            } else {
                // delete least used
                int keyToDel = l.back().first;
                m.erase(keyToDel);
                l.pop_back();
                
                // insert new
                l.push_front(make_pair(key, value));
                m[key] = l.begin();
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
