class DinnerPlates {
private:
    int capa;
    map<int, int> empties;
    vector<vector<int>> stacks; 
public:
    DinnerPlates(int capacity) {
        capa = capacity;
        empties.clear();
        stacks.clear();
    }
    
    void push(int val) {
        if (empties.empty()) {
            if (stacks.empty() 
                || stacks.back().size() >= capa) {
                stacks.push_back(vector<int>());
            }
            stacks.back().push_back(val);
        } else {
            int index = empties.begin()->first;
            int& count = empties.begin()->second;
            count--;
            stacks[index].push_back(val);
            if (count <= 0) {
                empties.erase(empties.begin());
            }
        }
    }
    
    int pop() {
        if (stacks.empty()) {
            return -1;
        }
        return popAtStack(stacks.size() - 1);
    }
    
    int popAtStack(int index) {
        if (index >= stacks.size() 
            || stacks[index].size() == 0) {
            return -1;
        } else if (index == stacks.size() - 1) {
            map<int, int>::iterator it = empties.find(index);
            if (it != empties.end()) {
                empties.erase(it);
            }
            int res = stacks.back().back();
            stacks.back().pop_back();
            while (!stacks.empty() && stacks.back().empty()) {
                int i = stacks.size() - 1;
                map<int, int>::iterator it = empties.find(i);
                if (it != empties.end()) {
                    empties.erase(it);
                }
                stacks.pop_back();
            }
            return res;
        } else {
            empties[index]++;
            int res = stacks[index].back();
            stacks[index].pop_back();
            return res;
        }
    }
};

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * DinnerPlates* obj = new DinnerPlates(capacity);
 * obj->push(val);
 * int param_2 = obj->pop();
 * int param_3 = obj->popAtStack(index);
 */
