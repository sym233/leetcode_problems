/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

/**
 * Encodes a tree to a single string.
 *
 * @param {TreeNode} root
 * @return {string}
 */
var serialize = function(root) {
  const res = [];
  const queue = [root];
  while(queue.length !== 0){
    const node = queue.shift();
    if(node === null){
      res.push(null);
    }else{
      res.push(node.val);
      queue.push(node.left);
      queue.push(node.right);
    }
  }
  console.log(res);
  return res.toString();
};

/**
 * Decodes your encoded data to tree.
 *
 * @param {string} data
 * @return {TreeNode}
 */
var deserialize = function(data) {
  if(data === ''){
    return null;
  }
  const ar = data.split(',');

  const tree = new TreeNode(Number.parseInt(ar[0]));
  ar.shift();
  const queue = [tree];
  while(ar.length !== 0){
    const node = queue.shift();
    // left
    if(ar[0] === ''){
      node.left = null;
      ar.shift();
    }else{
      const left = new TreeNode(Number.parseInt(ar[0]));
      node.left = left;
      queue.push(left);
      ar.shift();
    }
    // right
    if(ar[0] === ''){
      node.right = null;
      ar.shift();
    }else{
      const right = new TreeNode(Number.parseInt(ar[0]));
      node.right = right;
      queue.push(right);
      ar.shift();
    }
  }
  return tree;
};

/**
 * Your functions will be called as such:
 * deserialize(serialize(root));
 */
