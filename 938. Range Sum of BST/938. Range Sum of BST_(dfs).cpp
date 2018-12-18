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
        
        
        if (L <= root->val && root->val <= R) {
            sum += root->val;
        }
        
        if (root->val < R && root->right != nullptr) {
            sum += rangeSumBST(root->right, L, R);
        }
        
        if (L < root->val && root->left != nullptr) {
            sum += rangeSumBST(root->left, L, R);
        }
        return sum;
    }
};
