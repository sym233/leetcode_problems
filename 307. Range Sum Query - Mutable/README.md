307. Range Sum Query - Mutable

I use a tree structure to store data, it's made by myself. It may look like segment tree but I haven't learned it yet.

TreeNode structure:

```
TreeNode{
    Number val;
    Number sum;
    Bool changed;
    Number leftCount;
    Number rightCount;
    TreeNode left;
    TreeNode right;
}
```

Array to tree, split array into 3 parts:

```
newTree([...leftPart, midElem, ...rightPart]) -> TreeNode{
	val = midElem;
	changed = false;
	leftCount = leftPart.length;
	rightCount = rightPart.length;
	left = newTree(leftPart);
	right = NewTree(rightPart);
	sum = val + left.sum + right.sum;
}
```

It's obviously that we can retrieve the origin array by mid-order Traversal of the tree. 

The sum attribute of a note is the sum of its val and all of its children. With leftCount and rightCount attribute we can easily visit the ith element of origin array.

update value:

Assign a pointer of root firstly. Then set pointed node's changed attribute to true. If pointed node isn't the one to update, move the pointer to its left or right child and redo that.

changed attribute is true means this node's value has been updated but sum hasn't not.

maintain:

If a note is changed, calculate its sum with its children and assign changed to false. If not, just return its sum to father and don't need to visit its children. It's a recursive procedure. Maintanence on each updates is not neccesary, do it just before a sum calculation is required.

sumRange:

Find notes which children cover the range, the sum of their sums is what we need.
