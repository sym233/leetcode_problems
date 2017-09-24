class MyTreeNode{
  constructor(nums){
    const l = nums.length;
    const m = l >> 1;
    this.val = nums[m];
    this.changed = false;
    this.sum = this.val;
    if(m === 0){
      this.leftCount = 0;
      this.left = null;
    }else{
      this.leftCount = m;
      this.left = new MyTreeNode(nums.slice(0, m));
      this.sum += this.left.sum;
    }
    if(m === l-1){
      this.rightCount = 0;
      this.right = null;
    }else{
      this.rightCount = l-m-1;
      this.right = new MyTreeNode(nums.slice(m+1));
      this.sum += this.right.sum;
    }
  }
}

class MyTree{
/**
 * @param {number[]} nums
 */
  constructor(nums){
    if(nums.length){
      this.root = new MyTreeNode(nums);
    }else{
      this.root = null;
    }
    
    this.changed = false;
  }
  
/** 
 * @param {number} i 
 * @param {number} val
 * @return {void}
 */
  update(i, val){
    this.changed = true;
    let p = this.root;
    for(;;){
      console.log(i, p.val);
      if(p === null){
        throw new Error('can\'t find this node');
      }
      p.changed = true;
      if(p.leftCount === i){
        p.val = val;
        break;
      }else if(i < p.leftCount){
        p = p.left;
      }else{
        i -= p.leftCount + 1;
        p = p.right;
      }
    }
  }
  maintain(){
    if(this.changed){
      this.change = false;
      function getSum(node){
        if(node === null){
          return 0;
        }
        if(node.changed){
          node.sum = getSum(node.left) + node.val + getSum(node.right);
          node.change = false;
          return node.sum;
        }else{
          return node.sum;
        }
      }
      getSum(this.root);
    }
  }

/** 
 * @param {number} i 
 * @param {number} j
 * @return {number}
 */
  sumRange(i, j){
    this.maintain();
    function getSum(node, i, j){
      if(i === j && i === node.leftCount){
        return node.val;
      }
      if(i === 0 && j === (node.leftCount + node.rightCount)){
         return node.sum;
      }
      if(node.leftCount < i){
        // mid to the left of [i, j]
        return getSum(node.right, i - node.leftCount - 1, j - node.leftCount - 1);
      }else if(node.leftCount === i){
        // mid === i
        return node.val + getSum(node.right, 0, j - node.leftCount - 1);
      }else if(i < node.leftCount && node.leftCount < j){
        // mid in [i, j]
        return getSum(node.left, i, node.leftCount - 1) + node.val + getSum(node.right, 0, j - node.leftCount - 1);
      }else if(j === node.leftCount){
        // mid === j
        return getSum(node.left, i, node.leftCount - 1) + node.val;
      }else if(j < node.leftCount){
        // mid to the right of [i, j]
        return getSum(node.left, i, j); 
      }
      
      throw new Error('out of cases');
    }
    return getSum(this.root, i, j);
  }
}

var NumArray = MyTree;

/** 
 * Your NumArray object will be instantiated and called as such:
 * var obj = Object.create(NumArray).createNew(nums)
 * obj.update(i,val)
 * var param_2 = obj.sumRange(i,j)
 */
