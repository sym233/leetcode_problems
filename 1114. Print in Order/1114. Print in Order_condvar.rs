struct Foo {
    conds: Vec<Arc<(Mutex<i32>, Condvar)>>,
}

impl Foo {
    fn new() -> Self {
        let cond = Arc::new((Mutex::new(1), Condvar::new()));
        let conds = vec![cond.clone(); 3];
        Foo {
            conds,
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        let (num, cvar) = &*self.conds[0];
        let mut num = num.lock().unwrap();

        // Do not change this line
        print_first();

        *num = 2;
        cvar.notify_all();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let (num, cvar) = &*self.conds[1];
        let mut num = cvar.wait_while(num.lock().unwrap(), |n| { *n != 2 }).unwrap();

        // Do not change this line
        print_second();

        *num = 3;
        cvar.notify_all();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let (num, cvar) = &*self.conds[2];
        let _num = cvar.wait_while(num.lock().unwrap(), |n| { *n != 3 }).unwrap();

        // Do not change this line
        print_third();
    }
}
