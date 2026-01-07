use std::sync::mpsc::{channel, Sender, Receiver};

struct Foo {
    c1s: Sender<()>,
    c1r: Mutex<Receiver<()>>,
    c2s: Sender<()>,
    c2r: Mutex<Receiver<()>>,
}

impl Foo {
    fn new() -> Self {
        let (c1s, c1r) = channel();
        let (c2s, c2r) = channel();

        Foo {
            c1s,
            c1r: Mutex::new(c1r),
            c2s,
            c2r: Mutex::new(c2r),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {

        // Do not change this line
        print_first();

        self.c1s.send(()).unwrap();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        self.c1r.lock().unwrap().recv().unwrap();

        // Do not change this line
        print_second();

        self.c2s.send(()).unwrap();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        self.c2r.lock().unwrap().recv().unwrap();

        // Do not change this line
        print_third();
    }
}
