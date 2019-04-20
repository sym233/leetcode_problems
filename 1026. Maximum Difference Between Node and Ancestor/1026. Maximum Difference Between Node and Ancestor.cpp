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
    int max = 0;
    void update(int m) {
        if (m > max) {
            max = m;
        }
    }
    void update(pair<int, int>& p, int v) {
        if (v < p.first) {
            p.first = v;
        }
        if (v > p.second) {
            p.second = v;
        }
    }
    pair<int, int> getBound(TreeNode* node) {
        pair<int, int> bounds(node->val, node->val);
        if (node->left != nullptr) {
            pair<int, int> p = getBound(node->left);
            update(bounds, p.first);
            update(bounds, p.second);
        }
        if (node->right != nullptr) {
            pair<int, int> p = getBound(node->right);
            update(bounds, p.first);
            update(bounds, p.second);
        }
        
        update(abs(node->val - bounds.first));
        update(abs(node->val - bounds.second));
        return bounds;

    }
public:
    int maxAncestorDiff(TreeNode* root) {
        getBound(root);
        return max;
    }
};
