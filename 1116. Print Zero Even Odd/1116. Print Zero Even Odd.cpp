class ZeroEvenOdd {
private:
    int n;

    atomic<bool> print_zero = true;
    atomic<int> num =  1;
public:
    ZeroEvenOdd(int n) {
        this->n = n;
    }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        while (num <= n) {
            if (print_zero) {
                printNumber(0);
                print_zero = false;
            } else {
                this_thread::yield();
            }
        }
    }

    void even(function<void(int)> printNumber) {
        while (num <= n) {
            if (!print_zero && num % 2 == 0) {
                printNumber(num);
                print_zero = true;
                num++;
            } else {
                this_thread::yield();
            }
        }
    }

    void odd(function<void(int)> printNumber) {
        while (num <= n) {            
            if (!print_zero && num % 2 == 1) {
                printNumber(num);
                print_zero = true;
                num++;
            } else {
                this_thread::yield();
            }
        }
    }
};
