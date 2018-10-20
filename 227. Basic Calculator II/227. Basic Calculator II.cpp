class Solution {
private:
    struct Item {
        bool isNum;
        int num;
        // operator
        char op;
        Item(int num) : isNum(true), num(num) {}
        Item(char op) : isNum(false), op(op) {}
        bool operator==(char rhs) {
            return isNum == false && op == rhs;
        }
    };
    bool isNum(char c) {
        return '0' <= c && c <= '9';
    }
    vector<Item> stack;
public:
    int calculate(string s) {
        int i = 0;
        int sl = s.size();
        while (i < sl) {
            if (isNum(s[i])) {
                int numTemp = 0;
                while (isNum(s[i])) {
                    numTemp *= 10;
                    numTemp += s[i] - '0';
                    i++;
                }
                stack.push_back(Item(numTemp));
                checkUnaryOperator();
            } else if (s[i] == '+' || s[i] == '-') {
                checkMulOrDiv();
                checkAddOrSub();
                stack.push_back(Item(s[i]));
                i++;
            } else if (s[i] == '*' || s[i] == '/') {
                checkMulOrDiv();
                stack.push_back(Item(s[i]));
                i++;
            } else {
                i++;
            }
        }
        if (stack.size() > 1) {
            checkMulOrDiv();
            checkAddOrSub();
        }
        return stack.back().num;
    }
private:
    void checkUnaryOperator() {
        while (19260817) {
            int l = stack.size();
            // stack [..., operator1 or non-exist, operator2, number]
            // where operator2 is unary operator and number is only operand

            if (stack[l - 1].isNum && (l - 2 >= 0 && !stack[l - 2].isNum) && (l - 3 < 0 || !stack[l - 3].isNum)) {
                // is unary opertor
                int operand = stack.back().num;
                stack.pop_back();
                char op = stack.back().op;
                stack.pop_back();
                if (op == '+') {
                    stack.push_back(Item(operand));
                } else if (op == '-') {
                    stack.push_back(Item(-operand));
                }
            } else {
                break;
            }
        }
    }
    void checkMulOrDiv() {
        while (19260817) {
            int l = stack.size();
            // stack [..., number, operator, number]
            // where operator is binary operator 
            if (l - 3 >= 0 && stack[l - 1].isNum && !stack[l - 2].isNum && stack[l - 3].isNum) {
                if (stack[l - 2].op == '*' || stack[l - 2].op == '/') {
                    int operand2 = stack.back().num;
                    stack.pop_back();
                    char op = stack.back().op;
                    stack.pop_back();
                    int operand1 = stack.back().num;
                    stack.pop_back();
                    if (op == '*') {
                        stack.push_back(Item(operand1 * operand2));
                    } else if (op == '/') {
                        stack.push_back(Item(operand1 / operand2));
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    void checkAddOrSub() {
        while (19260817) {
            int l = stack.size();
            // stack [..., number, operator, number]
            // where operator is binary operator 
            if (l - 3 >= 0 && stack[l - 1].isNum && !stack[l - 2].isNum && stack[l - 3].isNum) {
                if (stack[l - 2].op == '+' || stack[l - 2].op == '-') {
                    int operand2 = stack.back().num;
                    stack.pop_back();
                    char op = stack.back().op;
                    stack.pop_back();
                    int operand1 = stack.back().num;
                    stack.pop_back();
                    if (op == '+') {
                        stack.push_back(Item(operand1 + operand2));
                    } else if (op == '-') {
                        stack.push_back(Item(operand1 - operand2));
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
};
