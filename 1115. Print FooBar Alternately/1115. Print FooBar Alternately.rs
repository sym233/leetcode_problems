struct FooBar {
    n: usize,
    is_foo: Arc<Mutex<bool>>,
    condvar: Arc<Condvar>,
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            n,
            is_foo: Arc::new(Mutex::new(true)),
            condvar: Arc::new(Condvar::new()),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut is_foo_val = self.condvar.wait_while(self.is_foo.lock().unwrap(), |b| { !*b }).unwrap();
            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();
            *is_foo_val = false;
            self.condvar.notify_one();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut is_foo_val = self.condvar.wait_while(self.is_foo.lock().unwrap(), |b| { *b }).unwrap();
            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();
            *is_foo_val = true;
            self.condvar.notify_one();
        }
    }
}
