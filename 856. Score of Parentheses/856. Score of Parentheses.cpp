struct Item {
    bool isParenthese;
    int val;
    char c;
    Item(bool isParenthese, int val, char c) : isParenthese(isParenthese), val(val), c(c) {};
};

class Solution {
public:
    int scoreOfParentheses(string S) {
        vector<Item> stack;

        for (char c : S) {
            if (c == '('){
                stack.push_back(Item(true, 0, '('));
            } else {
                Item& tail = stack.back();
                if (tail.isParenthese && tail.c == '(') {
                    stack.pop_back();
                    stack.push_back(Item(false, 1, 0));
                } else {
                    int score = 0;
                    while (stack.size() && !stack.back().isParenthese) {
                        score += stack.back().val;
                        stack.pop_back();
                    }
                    if (stack.back().c == '(') {
                        stack.pop_back();
                        stack.push_back(Item(false, 2 * score, 0));
                    } else {
                        cout << "unexpected ')'" << endl;
                        return -1;
                    }
                }
            }
        }
        
        int score = 0;
        while (stack.size() && !stack.back().isParenthese) {
            score += stack.back().val;
            stack.pop_back();
        }
        
        if (stack.size() > 0) {
            cout << "stack not empty" << endl;
            return -1;
        }
        
        return score;
    }
};
