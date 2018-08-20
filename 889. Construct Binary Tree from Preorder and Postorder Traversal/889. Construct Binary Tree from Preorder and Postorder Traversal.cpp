/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */


TreeNode* getRoot(vector<int>& pre, int preS, int preE, vector<int>& post, int postS, int postE) {
    if (preE - preS != postE - postS) {
        cout << "error! Not same length." << endl;
        return nullptr;
    }
    
    int l = preE - preS;
    if (l == 0) {
        // no node;
        return nullptr;
    }
    
    int rootVal = pre[preS];
    
    if (rootVal == post[postE - 1]) {
        
        TreeNode* root = new TreeNode(rootVal);
        
        // default left node
        if (l >= 2) {
            int leftRoot = pre[preS + 1];
            
            int leftLength = 0;
            int seperator;
            
            // find left root in post[]
            for (seperator = postS; seperator < postE; seperator++) {
                leftLength++;
                if (post[seperator] == leftRoot) {
                    break;
                }
            }
            
            root->left = getRoot(pre, preS + 1, preS + 1 + leftLength, post, postS, seperator + 1);
            
            if (seperator != postE - 2) {
                // seperator does not point to the second last element
                // it has right nodes
                
                root->right = getRoot(pre, preS + 1 + leftLength, preE, post, seperator + 1, postE - 1);
            }
            
        }
        
        return root;
        
    } else {
        cout << "error not found root" << endl;
        return nullptr;
	}  
}



class Solution {
public:
    TreeNode* constructFromPrePost(vector<int>& pre, vector<int>& post) {
        int l = pre.size();
        
        return getRoot(pre, 0, l, post, 0, l);
    }
};
