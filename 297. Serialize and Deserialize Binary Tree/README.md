297. Serialize and Deserialize Binary Tree

Serialize: bfs, push root into queue at first, shift a node and push it children if it isn't null until queue empty.

Serializing result: [root, ...level 2, ...level 3, ...]

Deserialize: create root and push it into queue, delete it from data.

Shift a node from queue, create two node as its children from the first 2 element of data. Push 2 children node into queue until they are not null. Do it until queue empty. 

Then a tree is created.
