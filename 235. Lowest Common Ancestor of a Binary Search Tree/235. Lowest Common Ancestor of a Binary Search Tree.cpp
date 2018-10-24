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
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (p == q) {
            return p;
        }
        
        if (p == root || q == root) {
            return root;
        }
        
        if (p->val < root->val) {
            if (q->val < root->val) {
                return lowestCommonAncestor(root->left, p, q);
            } else {
                return root;
            }
        } else {
            if (q->val < root->val) {
                return root;
            } else {
                return lowestCommonAncestor(root->right, p, q);
            }
        }
    }
};
