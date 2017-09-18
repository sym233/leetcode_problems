331. Verify Preorder Serialization of a Binary Tree

preorder: root, leftchild, right.

If a tree is not empty, the root can't be null.

Consider a TreeNode, it occupies a child place but provides 2 child places.

A null occupies a child place.

So assign a counter of child place initialized to 1. Count every node, a new node +1, a null -1. If counter become 0 or negative but nodes still remaining, it must be an invalid serialization. 

If counter >0 but no node left, it must also be an invalid serialization.
