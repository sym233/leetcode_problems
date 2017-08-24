class MedianFinder{
/**
 * initialize your data structure here.
 */
    constructor(){
        this.arr = [];
        this.count = 0;   
    }
    
/** 
 * @param {number} num
 * @return {void}
 */
    addNum(num){
        if(this.count === 0){
            this.count++;
            this.arr.push(num);
            return;
        }else{
            if(num <= this.arr[0]){
                this.arr.unshift(num);
                this.count++;
                return;
            }
            if(this.arr[this.count-1] <= num){
                this.arr.push(num);
                this.count++;
                return;
            }
            
            // binary search
            let l = 0;
            let r = this.count;
            let p = (l + r) >> 1;
            for(;;){
                if(this.arr[p] <= num && num <= this.arr[p+1]){
                    this.arr.splice(p+1, 0, num);
                    this.count++;
                    return;
                }
                if(this.arr[p] > num){
                    // search left
                    r = p;
                    p = (l + r) >> 1;
                }
                if(this.arr[p+1] < num){
                    // search right
                    l = p;
                    p = (l + r) >> 1;
                }
            }
        }
    }
/**
 * @return {number}
 */
    findMedian(){
        console.log(this.arr)
        if(this.count & 1){
            // odd
            const mid = this.count >> 1;
            return this.arr[mid];
        }else{
            // even
            const mid = this.count >> 1;
            return (this.arr[mid-1]+this.arr[mid])/2;
        }
    }
}

/** 
 * Your MedianFinder object will be instantiated and called as such:
 * var obj = Object.create(MedianFinder).createNew()
 * obj.addNum(num)
 * var param_2 = obj.findMedian()
 */
