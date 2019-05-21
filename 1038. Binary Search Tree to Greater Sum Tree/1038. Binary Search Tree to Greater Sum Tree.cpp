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
    void traversal(TreeNode* root, function<void(int&)> f) {
        if (root != nullptr) {
            traversal(root->left, f);
            f(root->val);
            traversal(root->right, f);
        }
    }
public:
    TreeNode* bstToGst(TreeNode* root) {
        vector<int> values;
        
        traversal(root, [&values] (int& value) {
            values.push_back(value);
        });
        
        const int l = values.size();
        for (int i = l - 2; i >= 0; i--) {
            values[i] += values[i + 1];
        }
        
        int i = 0;
        traversal(root, [&values, &i] (int& value) {
            value = values[i];
            i++;
        });
        
        return root;
    }
};
