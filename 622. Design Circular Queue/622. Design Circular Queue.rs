struct MyCircularQueue {
    q: Vec<i32>,
    size: usize,
    count: usize,
    front: usize,
    rear: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        let k = k as usize;
        return Self {
            q: vec![0; k],
            size: k,
            count: 0,
            front: 0,
            rear: 0,
        };
    }
    
    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.q[self.rear] = value;
        self.rear = (self.rear + 1) % self.size;
        self.count += 1;
        return true;
    }
    
    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.size;
        self.count -= 1;
        return true;
    }
    
    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.q[self.front];
    }
    
    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        if self.rear == 0 {
            return self.q[self.size - 1];
        } else {
            return self.q[self.rear - 1];
        }
    }
    
    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        return self.count == 0;
    }
    
    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        return self.count == self.size;
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
