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
    int countNodes(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        int depth = 0;
        TreeNode* p = root;
        while (p != nullptr) {
            p = p->left;
            depth++;
        }
        int width = 1 << (depth - 1);
        int s = 0;
        int e = width;
        while ((e - s) > 1) {
            int m = (s + e) / 2;
            int d = 0;
            TreeNode* p = root;
            while (p != nullptr) {
                d++;
                if (d == depth) {
                    break;
                }
                if (m & (1 << (depth - d - 1))) {
                    p = p->right;
                } else {
                    p = p->left;
                }
            }
            
            if (d == depth) {
                s = m;
            } else {
                e = m;
            }
        }
        return width - 1 + e;
    }
};
