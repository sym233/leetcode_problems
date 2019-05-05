/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* constructMaximumBinaryTree(vector<int>& nums) {
        return makeTree(nums.begin(), nums.end());
    }
private:
    using VI = vector<int>::iterator;
    TreeNode* makeTree(VI begin, VI end) {
        if (begin == end) {
            return nullptr;
        }
        VI maxPos = begin;
        int max = *begin;
        for (VI it = begin; it != end; it++) {
            if (*it > max) {
                maxPos = it;
                max = *it;
            }
        }
        TreeNode* t = new TreeNode(max);
        t->left = makeTree(begin, maxPos);
        t->right = makeTree(++maxPos, end);
        return t;
    }
};
