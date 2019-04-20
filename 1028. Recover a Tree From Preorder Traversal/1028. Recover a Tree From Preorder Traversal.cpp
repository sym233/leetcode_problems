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
    using NodeData = pair<int, int>;
    using NodeDatas = vector<NodeData>;
    NodeData parseNode(string& s, int begin, int end) {
        // return pair<int, int> for nodeData
        NodeData p(0, 0);
        for (int i = begin; i < end; i++) {
            if (s[i] == '-') {
                p.first++;
            } else {
                p.second *= 10;
                p.second += s[i] - '0';
            }
        }        
        return p;
    }
    NodeDatas parseData(string s) {
        NodeDatas nodeDatas;
//         regex is slow
//         regex re("-*\\d+");
//         regex_iterator<string::iterator> it(s.begin(), s.end(), re);
//         regex_iterator<string::iterator> rend;
//         for (; it != rend; it++) {
//             nodeDatas.push_back(parseNode(it->str()));
//         }
        
        int sliceBegin = 0;
        for (int i = 1; i < s.size(); i++) {
            if (s[i] == '-' && s[i - 1] != '-') {
                // nodeDatas.push_back(parseNode(s.substr(sliceBegin, i - sliceBegin)));
                nodeDatas.push_back(parseNode(s, sliceBegin, i));
                sliceBegin = i;
            }
        }
        // nodeDatas.push_back(parseNode(s.substr(sliceBegin, s.size() - sliceBegin)));
        nodeDatas.push_back(parseNode(s, sliceBegin, s.size()));
        return nodeDatas;
    }
    TreeNode* recover(int depth, NodeDatas& nodeDatas, int begin, int end) {
        if (end <= begin) {
            return nullptr;
        }
        TreeNode* node = new TreeNode(nodeDatas[begin].second);
        int rightChildBegin = end;
        for (int i = begin + 2; i < end; i++) {
            if (nodeDatas[i].first == depth + 1) {
                rightChildBegin = i;
                break;
            }
        }
        node->left = recover(depth + 1, nodeDatas, begin + 1, rightChildBegin);
        node->right = recover(depth + 1, nodeDatas, rightChildBegin, end);
        return node;
    }
public:
    TreeNode* recoverFromPreorder(string S) {
        NodeDatas nodeDatas = parseData(S);
        // nodeData[i].first depth
        // nodeData[i].second node value
        
        return recover(0, nodeDatas, 0, nodeDatas.size());
    }
};
