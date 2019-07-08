auto incr = [] (int a, int b, int c, int d) -> bool {
    return a <= b && b <= c && c <= d;
};

template <typename T>
struct MyTreeNode {
    int start;
    int end;
    T value;
    MyTreeNode* left;
    MyTreeNode* right;
    MyTreeNode(int start, int end, T value) : start(start), end(end), value(value) {
        left = nullptr;
        right = nullptr;
    }
    void insert(MyTreeNode* newNode) {
        if (newNode->end <= start) {
            // new |---|
            // old       |---|
            if (left == nullptr) {
                left = newNode;
                return;
            } else {
                return left->insert(newNode);
            }
        } else if (end <= newNode->start) {
            // new      |---|
            // old |---|
            if (right == nullptr) {
                right = newNode;
                return;
            } else {
                return right->insert(newNode);
            }
        } else if (incr(newNode->start, start, newNode->end, end)) {
            // new |-----|
            // old    |-----|
            int oldEnd = end;
            end = newNode->end;
            if (newNode->start != start) {
                insert(new MyTreeNode(newNode->start, start, newNode->value));
            }
            if (newNode->end != oldEnd) {
                insert(new MyTreeNode(newNode->end, oldEnd, value));
            }
            value += newNode->value;
            delete newNode;
            return;
        } else if (incr(newNode->start, start, end, newNode->end)) {
            // new |----------|
            // old     |---|
            if (newNode->start != start) {
                insert(new MyTreeNode(newNode->start, start, newNode->value));
            }
            if (end != newNode->end) {
                insert(new MyTreeNode(end, newNode->end, newNode->value));
            }
            value += newNode->value;
            delete newNode;
            return;
        } else if (incr(start, newNode->start, end, newNode->end)) {
            // new    |-----|
            // old |-----|
            int oldStart = start;
            start = newNode->start;
            if (oldStart != newNode->start) {
                insert(new MyTreeNode(oldStart, newNode->start, value));
            }
            if (end != newNode->end) {
                insert(new MyTreeNode(end, newNode->end, newNode->value));
            }
            value += newNode->value;
            delete newNode;
            return;
        } else if (incr(start, newNode->start, newNode->end, end)) {
            // new    |----|
            // old |----------|
            int oldStart = start;
            int oldEnd = end;
            start = newNode->start;
            end = newNode->end;
            if (oldStart != newNode->start) {
                insert(new MyTreeNode(oldStart, newNode->start, value));
            }
            if (newNode->end != oldEnd) {
                insert(new MyTreeNode(newNode->end, oldEnd, value));
            }
            value += newNode->value;
            delete newNode;
            return;
        }
    }
    void preOrder(function<void(int, int, T)> f) {
        if (left != nullptr) {
            left->preOrder(f);
        }
        f(start, end, value);
        if (right != nullptr) {
            right->preOrder(f);
        }
    }
};

template <typename T>
class MyTree {
private:
    MyTreeNode<T>* root = nullptr;
public:
    void add(int s, int e, T v) {
        MyTreeNode<T>* n = new MyTreeNode<T>(s, e, v);
        if (root == nullptr) {
            root = n;
        } else {
            root->insert(n);
        }
    }
    void preOrder(function<void(int, int, T)> f) {
        if (root != nullptr) {
            root->preOrder(f);
        }
    }
};

class Solution {
public:
    bool carPooling(vector<vector<int>>& trips, int capacity) {
        MyTree<int> myTree;
        bool res = true;
        function<void(int, int, int)> test = [capacity, &res](int s, int e, int v) -> void {
            if (v > capacity) {
                res = false;
            }
        };
        for (vector<int>& trip : trips) {
            myTree.add(trip[1], trip[2], trip[0]);
        }
        myTree.preOrder(test);
        return res;
    }
};
