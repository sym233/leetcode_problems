class SnapshotArray {
private:
    int sid = 0;
    vector<map<int, int>> vec;
    map<int, int> changes;
public:
    SnapshotArray(int length) {
        vec = vector<map<int, int>>(length);
        for (int i = 0; i < length; i++) {
            changes[i] = 0;
        }
    }
    
    void set(int index, int val) {
        changes[index] = val;
    }
    
    int snap() {
        for (auto it = changes.begin(); it != changes.end(); it++) {
            vec[it->first][sid] = it->second;
        }
        changes.clear();
        return sid++;
    }
    
    int get(int index, int snap_id) {
        map<int, int>& m = vec[index];
        map<int, int>::iterator it = m.lower_bound(snap_id);
        if (it == m.end() || it->first != snap_id) {
            it--;
        }
        return it->second;
    }
};

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * SnapshotArray* obj = new SnapshotArray(length);
 * obj->set(index,val);
 * int param_2 = obj->snap();
 * int param_3 = obj->get(index,snap_id);
 */
