class FooBar {
private:
    int n;
    atomic<bool> is_foo = true;
public:
    FooBar(int n) {
        this->n = n;
    }

    void foo(function<void()> printFoo) {
        for (int i = 0; i < n; i++) {
            while (!is_foo) {
                this_thread::yield();
            }
            
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
            
            is_foo = false;
        }
    }

    void bar(function<void()> printBar) {
        for (int i = 0; i < n; i++) {
            while (is_foo) {
                this_thread::yield();
            }
            
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
            
            is_foo = true;
        }
    }
};
