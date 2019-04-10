/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * class NestedInteger {
 *   public:
 *     // Constructor initializes an empty nested list.
 *     NestedInteger();
 *
 *     // Constructor initializes a single integer.
 *     NestedInteger(int value);
 *
 *     // Return true if this NestedInteger holds a single integer, rather than a nested list.
 *     bool isInteger() const;
 *
 *     // Return the single integer that this NestedInteger holds, if it holds a single integer
 *     // The result is undefined if this NestedInteger holds a nested list
 *     int getInteger() const;
 *
 *     // Set this NestedInteger to hold a single integer.
 *     void setInteger(int value);
 *
 *     // Set this NestedInteger to hold a nested list and adds a nested integer to it.
 *     void add(const NestedInteger &ni);
 *
 *     // Return the nested list that this NestedInteger holds, if it holds a nested list
 *     // The result is undefined if this NestedInteger holds a single integer
 *     const vector<NestedInteger> &getList() const;
 * };
 */
class Solution {
private:
    int i = 0;
    int l = 0;
    bool isNumber(char c) {
        return '0' <= c && c <= '9';
    }
    int readInteger(string s) {
        int res = 0;
        int sign = 1;
        if (s[i] == '-') {
            sign = -1;
            i++;
        }
        while (i < l && isNumber(s[i])) {
            int n = s[i] - '0';
            res *= 10;
            res += n;
            i++;
        }
        return res * sign;
    }
public:
    NestedInteger deserialize(string s) {
        if (l == 0) {
            l = s.size();
        }
        if (s[i] == '[') {
            NestedInteger result = NestedInteger();
            i++;
            while (s[i] != ']') {
                result.add(deserialize(s));
                if (s[i] == ',') {
                    i++;
                }
            }
            i++;
            return result;
        } else {
            int n = readInteger(s);
            NestedInteger result = NestedInteger(n);
            return result;
        }
    }
};
