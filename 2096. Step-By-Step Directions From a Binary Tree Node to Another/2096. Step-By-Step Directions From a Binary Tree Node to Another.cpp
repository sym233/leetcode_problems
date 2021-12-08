/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    string getDirections(TreeNode* root, int startValue, int destValue) {
        this->startValue = startValue;
        this->destValue = destValue;
        string path = "";
        dfs(root, path);
        
        int minL = startPath.size();
        if (destPath.size() < minL) {
            minL = destPath.size();
        }
        int i = 0;
        for (; i < minL; i++) {
            if (startPath[i] != destPath[i]) {
                break;
            }
        }
        startPath = startPath.substr(i);
        destPath = destPath.substr(i);
        
        string res;
        for (auto _ : startPath) {
            res += 'U';
        }
        res += destPath;
        
        return res;
    }
private:
    int startValue;
    int destValue;
    bool startFound = false;
    bool destFound = false;
    string startPath = "";
    string destPath = "";
    void dfs(TreeNode* node, string& path) {
        if (node->val == startValue) {
            startFound = true;
            startPath = path;
        }
        if (node->val == destValue) {
            destFound = true;
            destPath = path;
        }
        if (!(startFound && destFound) && node->left != nullptr) {
            path += 'L';
            dfs(node->left, path);
            path.pop_back();
        }
        if (!(startFound && destFound) && node->right != nullptr) {
            path += 'R';
            dfs(node->right, path);
            path.pop_back();
        }
    }
};
