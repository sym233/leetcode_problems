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
    vector<Item> stack;
    
public:
    int calculate(string s) { 
        bool readingNumber = false;
        int numTemp = 0;
        for (char c : s) {
            if (c == ' ') {
                continue;
            } else if ('0' <= c && c <= '9') {
                readingNumber = true;
                numTemp *= 10;
                numTemp += c - '0';
            } else if (c == '+' || c == '-') {
                if (readingNumber) {
                    stack.push_back(Item(numTemp));
                    readingNumber = false;
                    numTemp = 0;
                    proceed();
                }
                stack.push_back((c));
            } else if (c == '(') {
                stack.push_back(Item(c));
            } else if (c == ')') {
                if (readingNumber) {
                    stack.push_back(Item(numTemp));
                    readingNumber = false;
                    numTemp = 0;
                    proceed();
                }
                stack.push_back(Item(c));
                proceed();
            } else {
                cerr << "unknown char " << c << endl;
                throw;
            }
        }
        if (readingNumber) {
            stack.push_back(Item(numTemp));
            readingNumber = false;
            numTemp = 0;
            proceed();
        }
        
        if (stack.size() == 1 && stack.back().isNum) {
            return stack.back().num;
        } else {
            throw;
        }
        return 0;
    }
private: 
    void proceed() {
        int l = stack.size();
        // stack[l - 1] last-pushed number
        // stack[l - 2] operator or parenthesis if exists
        // stack[l - 2] number, operator or parenthesis if exists
        bool needMoreProcess = false;
        
        Item& t1 = stack[l - 1];
        if (t1.isNum) {
            if (l - 2 >= 0) {
                Item& t2 = stack[l - 2];
                if (t2.isNum) {
                    cerr << "error, unexpected number " << t2.num << endl;
                    throw;
                } else if (t2 == '+' || t2 == '-') {
                    if (l - 3 >= 0) {
                        Item& t3 = stack[l - 3];
                        if (t3.isNum) {
                            // t2 '+' / '-' sign is addition / subtract
                            proceedOperation();
                            needMoreProcess = true;
                        } else if (t3 == '+' || t3 == '-' || t3 == '('){
                            // treat t2 '+' / '-' sign as positive / negative sign
                            proceedSign();
                            needMoreProcess = true;
                        } else if (t3 == ')') {
                            // not possible
                            cerr << "error, unexpected ')' " << endl;
                            throw;
                        } else {
                            // not possible
                            cerr << "error, unknown operator " << t2.op << endl;
                            throw;
                        }
                    } else {
                        // t2 is first item
                        // treat t2 '+' / '-' sign as positive / negative sign
                        proceedSign();
                        needMoreProcess = false;
                    }
                } else if (t2 == '(') {
                    needMoreProcess = false;
                } else if (t2 == ')') {
                    // not possible
                    cerr << "error, unexpected ')' " << endl;
                } else {
                    // not possible
                    cerr << "error, unknown operator " << t2.op << endl;
                    throw;
                }
            } else {
                // first item
                needMoreProcess = false;
            }
        } else if (t1 == ')') {
            if (l - 3 >= 0 && stack[l - 2].isNum && stack[l - 3] == '(') {
                proceedParenthesis();
                needMoreProcess = true;
            }
        } else {
            needMoreProcess = false;
        }
        
        if (needMoreProcess) {
            return proceed();
        }
    }
    void proceedSign() {
        int operand = stack.back().num;
        stack.pop_back();
        char op = stack.back().op;
        stack.pop_back();
        
        int res;
        if (op == '+') {
            res = operand;
        } else if (op == '-') {
            res = -operand;
        }
        stack.push_back(res);
    }
    void proceedOperation() {
        int operand2 = stack.back().num;
        stack.pop_back();
        char op = stack.back().op;
        stack.pop_back();
        int operand1 = stack.back().num;
        stack.pop_back();
        
        int res;
        if (op == '+') {
            res = operand1 + operand2;
        } else if (op == '-') {
            res = operand1 - operand2;
        }
        stack.push_back(res);
    }
    void proceedParenthesis() {
        stack.pop_back();
        int res = stack.back().num;
        stack.pop_back();
        stack.pop_back();
        
        stack.push_back(res);
    }
};
