class H2O {
    final Semaphore h = new Semaphore(2, true);
    final Semaphore o = new Semaphore(1, true);
    void addH() throws InterruptedException {
        h.acquire();
    }
    void addO() throws InterruptedException {
        o.acquire();
    }
    synchronized void reset() {
        int needH = h.availablePermits();
        int needO = o.availablePermits();
        if (needH == 0 && needO == 0) {
            h.release(2);
            o.release();
        } 
    }
    public void hydrogen(Runnable releaseHydrogen) throws InterruptedException {
		addH();
        // releaseHydrogen.run() outputs "H". Do not change or remove this line.
        releaseHydrogen.run();
        reset();
    }

    public void oxygen(Runnable releaseOxygen) throws InterruptedException {
        addO();
        // releaseOxygen.run() outputs "O". Do not change or remove this line.
		releaseOxygen.run();
        reset();
    }
}
