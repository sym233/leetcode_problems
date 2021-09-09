class FizzBuzz {
    private int n;
    private int i;
    private int token;

    public FizzBuzz(int n) {
        this.n = n;
        i = 1;
        token = nextToken(i);
    }
    
    private int nextToken(int num) {
        int t = 0;
        if (num % 3 == 0) {
            t += 1;
        }
        if (num % 5 == 0) {
            t += 2;
        }
        // t = 0 number
        //     1 fizz
        //     2 buzz
        //     3 fizzbuzz
        return t;
    }

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        while (i <= n) {
            if (token == 1) {
                printFizz.run();
                i++;
                token = nextToken(i);
            } else {
                Thread.yield();
            }
        }
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        while (i <= n) {
            if (token == 2) {
                printBuzz.run();
                i++;
                token = nextToken(i);
            } else {
                Thread.yield();
            }
        }
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        while (i <= n) {
            if (token == 3) {
                printFizzBuzz.run();
                i++;
                token = nextToken(i);
            } else {
                Thread.yield();
            }
        }
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        while (i <= n) {
            if (token == 0) {
                printNumber.accept(i);
                i++;
                token = nextToken(i);
            } else {
                Thread.yield();
            }
        }
    }
}
