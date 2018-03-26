/**
 * @param {number[][]} people
 * @return {number[][]}
 */
type aPeople = number[];
type Que = aPeople[];

const reconstructQueue = function (people: Que): Que {
    const length = people.length;

    if (length === 0) {
        return [];
    }

    people.sort((a, b) => {
        if (b[0] !== a[0]) {
            return b[0] - a[0];
        } else {
            return a[1] - b[1];
        }
    });

    class BST_Node<T>{
        public val: T;
        public left: BST_Node<T>;
        public right: BST_Node<T>;
        public leftCount = 0;
        public rightCount = 0;

        constructor(val: T) {
            this.val = val;
            this.left = this.right = null;
        }
    }
    class BST<T>{
        public root: BST_Node<T> = null;
        insertByCount(node: BST_Node<T>, index: number) {
            if (this.root === null) {
                if (index === 0) {
                    this.root = node;
                } else {
                    throw new Error('node index error');
                }
            } else {
                let p = this.root;
                for (; ;) {
                    if (index <= p.leftCount) {
                        if (p.left !== null) {
                            p.leftCount++;
                            p = p.left;
                        } else {
                            p.leftCount++;
                            p.left = node;
                            return;
                        }
                    } else {
                        index -= p.leftCount + 1;
                        if (p.right !== null) {
                            p.rightCount++;
                            p = p.right;
                        } else {
                            p.rightCount++;
                            p.right = node;
                            return;
                        }
                    }
                }
            }
        }
        midTraversal(node = this.root): T[] {
            if(node === null){
                return [];
            }
            const res: T[] = [];
            if (node.left !== null) {
                res.push(...this.midTraversal(node.left));
            }
            res.push(node.val);
            if (node.right !== null) {
                res.push(...this.midTraversal(node.right));
            }
            return res;
        }
    }
    const t = new BST<aPeople>();
    for (const person of people) {
        t.insertByCount(new BST_Node(person), person[1]);
        // console.log(t.midTraversal());
    }
    return t.midTraversal();
};
