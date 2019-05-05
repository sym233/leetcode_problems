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
    TreeNode* insertIntoMaxTree(TreeNode* root, int val) {
        if (root == nullptr) {
            return new TreeNode(val);
        }
        if (root->val < val) {
            TreeNode* r = new TreeNode(val);
            r->left = root;
            return r;
        }
        TreeNode* p = root;
        while (p->right != nullptr) {
            if (p->right->val < val) {
                TreeNode* r = new TreeNode(val);
                r->left = p->right;
                p->right = r;
                return root;
            }
            p = p->right;
        }
        p->right = new TreeNode(val);
        return root;
    }
};
