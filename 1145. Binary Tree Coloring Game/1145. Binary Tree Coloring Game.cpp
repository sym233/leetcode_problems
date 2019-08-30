/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

function<int(TreeNode*)> countC = [](TreeNode* node) -> int {
    if (node == nullptr) {
        return 0;
    }
    return 1 + countC(node->left) + countC(node->right);
};
function<TreeNode*(TreeNode*, int)> findN = [](TreeNode* node, int val) -> TreeNode* {
    if (node == nullptr) {
        return nullptr;
    }
    if (node->val == val) {
        return node;
    }
    TreeNode* n = findN(node->left, val);
    if (n != nullptr) {
        return n;
    } else {
        return findN(node->right, val);
    }
};
class Solution {
public:
    bool btreeGameWinningMove(TreeNode* root, int n, int x) {
        TreeNode* node = findN(root, x);
        int lChildren = countC(node->left);
        if (lChildren > n / 2) {
            return true;
        }
        int rChildren = countC(node->right);
        if (rChildren > n / 2) {
            return true;
        }
        int others = n - lChildren - rChildren - 1;
        if (others > n / 2) {
            return true;
        }
        return false;
    }
};
