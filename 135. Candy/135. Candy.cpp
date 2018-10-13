class Solution {
public:
    int candy(vector<int>& ratings) {
        // assume each child has 1 candy at beginning
        struct ChildInfo {
            int indegree;
            int candies;
            vector<int> arcsTo;
            ChildInfo () : indegree(0), arcsTo(vector<int>()), candies(1) {}
        };
        
        int l = ratings.size();
        vector<ChildInfo> children(l);
        
        // If a child has a higher-rated neighbourhood
        // the former adds an arc to the latter.
        for (int i = 0; i < l; i++) {
            int left = i - 1;
            if (left >= 0 && ratings[left] > ratings[i]) {
                children[i].arcsTo.push_back(left);
                children[left].indegree++;
            }
            int right = i + 1;
            if (right < l && ratings[i] < ratings[right]) {
                children[i].arcsTo.push_back(right);
                children[right].indegree++;
            }
        }

        // children who has 0 indegree need only one candy
        queue<int> candyQueue;
        for (int i = 0; i < l; i++) {
            if (children[i].indegree == 0) {
                candyQueue.push(i);
            }
        }

        // use arcs to find other children who need more candies
        while (candyQueue.size() > 0) {
            int front = candyQueue.front();
            for (int a : children[front].arcsTo) {
                if (children[a].candies <= children[front].candies) {
                    children[a].candies = children[front].candies + 1;
                    candyQueue.push(a);
                }
            }
            candyQueue.pop();
        }
        
        int res = 0;
        for (ChildInfo child : children) {
            res += child.candies;
        }
        return res;
    }
};
