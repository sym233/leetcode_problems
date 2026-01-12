use std::sync::Condvar;

struct ZeroEvenOdd {
    n: i32,
    mutex_cv: Arc<(Mutex<(bool, i32)>, Condvar)>,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {

        ZeroEvenOdd {
            n,
            mutex_cv: Arc::new((Mutex::new((true, 1)), Condvar::new())),
        }
    }

    // printNumber(x) prints the integer x
    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let (mutex, cv) = &*self.mutex_cv;
            let mut guard = cv.wait_while(mutex.lock().unwrap(), |(is_zero, _)| !*is_zero).unwrap();

            print_number(0);

            guard.0 = false;
            cv.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n / 2 {
            let (mutex, cv) = &*self.mutex_cv;
            let mut guard = cv.wait_while(mutex.lock().unwrap(), |(is_zero, curr)| *is_zero || *curr % 2 != 0).unwrap();

            print_number(guard.1);

            guard.0 = true;
            guard.1 += 1;
            cv.notify_all();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..(self.n + 1) / 2 {
            let (mutex, cv) = &*self.mutex_cv;
            let mut guard = cv.wait_while(mutex.lock().unwrap(), |(is_zero, curr)| *is_zero || *curr % 2 == 0).unwrap();

            print_number(guard.1);

            guard.0 = true;
            guard.1 += 1;
            cv.notify_all();
        }
    }
}
