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
    int rangeSumBST(TreeNode* root, int L, int R) {
        if (root == nullptr) {
            return 0;
        }
        
        int sum = 0;
        
        queue<TreeNode*> q;
        q.push(root);
        
        
        while (!q.empty()) {
            TreeNode* p = q.front();
            if (L <= p->val && p->val <= R) {
                sum += p->val;
            }
            if (p->val < R && p->right != nullptr) {
                q.push(p->right);
            }
            if (L < p->val && p->left != nullptr) {
                q.push(p->left);
            }
            q.pop();
        }
        return sum;
    }
};
