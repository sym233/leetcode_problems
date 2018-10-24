class MyQueue {
private:
    vector<int> stackForTail;
    vector<int> stackForHead;
    bool elementsInTail; 
public:
    /** Initialize your data structure here. */
    MyQueue() {
        elementsInTail = true;
    }
    
    /** Push element x to the back of queue. */
    void push(int x) {
        if (!elementsInTail) {
            while (!stackForHead.empty()) {
                stackForTail.push_back(stackForHead.back());
                stackForHead.pop_back();
            }
            elementsInTail = true;
        }
        stackForTail.push_back(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    int pop() {
        if (elementsInTail) {
            while (!stackForTail.empty()) {
                stackForHead.push_back(stackForTail.back());
                stackForTail.pop_back();
            }
            elementsInTail = false;
        }
        int t = stackForHead.back();
        stackForHead.pop_back();
        return t;
    }
    
    /** Get the front element. */
    int peek() {
        if (elementsInTail) {
            while (!stackForTail.empty()) {
                stackForHead.push_back(stackForTail.back());
                stackForTail.pop_back();
            }
            elementsInTail = false;
        }
        return stackForHead.back();
    }
    
    /** Returns whether the queue is empty. */
    bool empty() {
        return stackForTail.empty() && stackForHead.empty();
    }
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue obj = new MyQueue();
 * obj.push(x);
 * int param_2 = obj.pop();
 * int param_3 = obj.peek();
 * bool param_4 = obj.empty();
 */
 