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
    TreeNode* lcaDeepestLeaves(TreeNode* root) {
        int max_lv = 0;
        TreeNode* res;
        
        function<int(TreeNode*, int)> get_lv = [&get_lv, &res, &max_lv](TreeNode* node, int lv) -> int {
            if (node == nullptr) {
                return lv - 1;
            }
            int left_lv = get_lv(node->left, lv + 1);
            int right_lv = get_lv(node->right, lv + 1);
            if (left_lv == right_lv) {
                if (left_lv >= max_lv){
                    res = node;
                    max_lv = left_lv;
                }
                return left_lv;
            } else {
                return left_lv > right_lv ? left_lv : right_lv;
            }
        };
        
        get_lv(root, 1);
        return res;
    }
};
