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
private:
    const int maxn = 1e4 + 1;
    vector<int> v;
    void f(TreeNode* node, int level) {
        if (node == nullptr) {
            return;
        }
        v[level] += node->val;
        f(node->left, level + 1);
        f(node->right, level + 1);
    }
public:
    int maxLevelSum(TreeNode* root) {
        v.resize(maxn, 0);
        f(root, 1);
        int maxv = 0;
        int maxi = 0;
        for (int i = 1; i < maxn; i++) {
            if (v[i] > maxv) {
                maxv = v[i];
                maxi = i;
            }
        }
        return maxi;
    }
};
