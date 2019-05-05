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
    using TN = TreeNode;
    TN* constructMaximumBinaryTree(vector<int>& nums) {
        stack<TN*> s;
        TN* root = new TN(nums[0]);
        s.push(root);
        const int l = nums.size();
        for (int i = 1; i < l; i++) {
            TN* node = new TN(nums[i]);
            while (nums[i] > s.top()->val) {
                s.pop();
                if (s.empty()) {
                    node->left = root;
                    root = node;
                    s.push(root);
                    goto endInsert;
                }
            }
            node->left = s.top()->right;
            s.top()->right = node;
            s.push(node);
            endInsert:;
        }
        
        return root;
    }
};
