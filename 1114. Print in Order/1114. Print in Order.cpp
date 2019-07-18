class Foo {
private:
    atomic<int> a = 0;
public:
    Foo() {
    }

    void first(function<void()> printFirst) {
        
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        
        a++;
    }

    void second(function<void()> printSecond) {
        while (a != 1) {
            this_thread::yield();
        }
        
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        
        a++;
    }

    void third(function<void()> printThird) {
        while (a != 2) {
            this_thread::yield();
        }
        
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
    }
};
