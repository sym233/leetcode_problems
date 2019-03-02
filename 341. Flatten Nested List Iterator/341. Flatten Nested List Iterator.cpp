/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * class NestedInteger {
 *   public:
 *     // Return true if this NestedInteger holds a single integer, rather than a nested list.
 *     bool isInteger() const;
 *
 *     // Return the single integer that this NestedInteger holds, if it holds a single integer
 *     // The result is undefined if this NestedInteger holds a nested list
 *     int getInteger() const;
 *
 *     // Return the nested list that this NestedInteger holds, if it holds a nested list
 *     // The result is undefined if this NestedInteger holds a single integer
 *     const vector<NestedInteger> &getList() const;
 * };
 */
class NestedIterator {
private:
    vector<int> nums;
    vector<int>::iterator it;
    void flatten(const vector<NestedInteger> &nestedList) {
        for (const NestedInteger& ni : nestedList) {
            if (ni.isInteger()) {
                nums.push_back(ni.getInteger());
            } else {
                const vector<NestedInteger>& nl = ni.getList();
                flatten(nl);
            }
        }
    }
public:
    NestedIterator(vector<NestedInteger> &nestedList) {
        flatten(nestedList);
        it = nums.begin();
    }

    int next() {
        return *(it++);
    }

    bool hasNext() {
        return it != nums.end();
    }
};

/**
 * Your NestedIterator object will be instantiated and called as such:
 * NestedIterator i(nestedList);
 * while (i.hasNext()) cout << i.next();
 */
 