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
    bool isCompleteTree(TreeNode* root) {
        if (root == nullptr) {
            return true;
        }
        
        bool meetNull = false;
        queue<TreeNode*> q;
        
        q.push(root);
        
        while (!q.empty()) {
            TreeNode* p = q.front();
            if (p == nullptr) {
                meetNull = true;
            } else {
                if (meetNull) {
                    return false;
                } else {
                    q.push(p->left);
                    q.push(p->right);
                }
            }
            q.pop();
        }
        return true;
    }
};
