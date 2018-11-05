class Solution {
private:
    vector<TreeNode*> findPath(TreeNode* root, TreeNode* p) {
        struct FatherIndex {
            // store tree node and its father's index in vector
            TreeNode* node;
            int fatherIndex;
            FatherIndex(TreeNode* node, int fatherIndex) : node(node), fatherIndex(fatherIndex) {}
        };
        struct NodeIndex {
            // store tree node and its index in vector
            TreeNode* node;
            int nodeIndex;
            NodeIndex(TreeNode* node, int nodeIndex) : node(node), nodeIndex(nodeIndex) {}
        };
        
        queue<NodeIndex> que({ NodeIndex(root, 0) });
        vector<FatherIndex> fathers({ FatherIndex(root, -1) });
        
        while (que.front().node != p) {
            if (que.front().node != nullptr) {
                TreeNode* left = que.front().node->left;
                fathers.push_back(FatherIndex(left, que.front().nodeIndex));
                que.push(NodeIndex(left, fathers.size() - 1));
                
                TreeNode* right = que.front().node->right;
                fathers.push_back(FatherIndex(right, que.front().nodeIndex));
                que.push(NodeIndex(right, fathers.size() - 1));
            }
            que.pop();
        }

        int i = que.front().nodeIndex;
        vector<TreeNode*> res;
        do {
            res.push_back(fathers[i].node);
            i = fathers[i].fatherIndex;
        } while (i != -1);
        return res;
    }

public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        vector<TreeNode*> pathP = findPath(root, p);
        vector<TreeNode*> pathQ = findPath(root, q);
        
        TreeNode* previousNode = nullptr;
        
        while (pathP.size() && pathQ.size() && pathP.back() == pathQ.back()) {
            previousNode = pathP.back();
            pathP.pop_back();
            pathQ.pop_back();
        }
        
        return previousNode;
    }
};
