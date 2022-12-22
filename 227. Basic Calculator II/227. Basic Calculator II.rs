use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Eq;

struct TreeNode<T> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    // priority
    fn p(&self) -> i32 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }
    // func
    fn f(&self) -> fn(i32, i32) -> i32 {
        match self {
            Op::Add => i32::add,
            Op::Sub => i32::sub,
            Op::Mul => i32::mul,
            Op::Div => i32::div,
        }
    }
}

impl From<u8> for Op {
    fn from(item: u8) -> Self {
        match item {
            b'+' => Op::Add,
            b'-' => Op::Sub,
            b'*' => Op::Mul,
            b'/' => Op::Div,
            _ => panic!("")
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Item {
    Op(Op),
    Num(i32),
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn wrap(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }
}

impl TreeNode<Item> {
    fn new_node(op: Op) -> Self {
        Self::new(Item::Op(op))
    }
    fn new_leaf(num: i32) -> Self {
        Self::new(Item::Num(num))
    }
}

type TN = TreeNode<Item>;
type Ast = Option<Rc<RefCell<TN>>>;

fn is_num(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}
fn tokenize(bytes: &[u8]) -> Vec<Item> {
    let mut tokens = vec![];
    let l = bytes.len();
    let mut i = 0;
    while i < l {
        match bytes[i] {
            b' ' => i += 1,
            op @ (b'+' | b'-' | b'*' | b'/') => {
                i += 1;
                tokens.push(Item::Op(Op::from(op)));
            },
            num if is_num(num) => {
                let mut j = i + 1;
                while j < l && is_num(bytes[j]) {
                    j += 1;
                }
                let num = std::str::from_utf8(&bytes[i..j]).unwrap().parse::<i32>().unwrap();
                tokens.push(Item::Num(num));
                i = j;
            },
            _ => panic!("unexpected char"),
        }
    }
    return tokens;
}

fn build_tree(tokens: &[Item]) -> Ast {
    let mut asts: Vec<Ast> = vec![];
    let mut ops: Vec<Op> = vec![];

    for token in tokens {
        match token {
            &Item::Num(num) => {
                asts.push(TN::new_leaf(num).wrap());
            },
            &Item::Op(op) => {
                while let Some(prev_op) = ops.last() {
                    if prev_op.p() >= op.p() {
                        let prev_op = ops.pop().unwrap();
                        let rhs = asts.pop().unwrap();
                        let lhs = asts.pop().unwrap();
                        let mut node = TN::new_node(prev_op);
                        node.left = lhs;
                        node.right = rhs;
                        asts.push(node.wrap());
                    } else {
                        break;
                    }
                }
                ops.push(op);
            }
        }
    }
    while let Some(op) = ops.pop() {
        let rhs = asts.pop().unwrap();
        let lhs = asts.pop().unwrap();
        let mut node = TN::new_node(op);
        node.left = lhs;
        node.right = rhs;
        asts.push(node.wrap());
    }
    return asts.pop().flatten();
}

fn eval(ast: &Ast) -> i32 {
    if let Some(ast) = ast {
        let node = ast.borrow();
        match node.val {
            Item::Op(op) => {
                return op.f()(eval(&node.left), eval(&node.right));
            },
            Item::Num(num) => {
                return num;
            },
        }
    } else {
        panic!("unexpected None ast");
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let tokens = tokenize(s.as_bytes());
        let ast = build_tree(&tokens);
        return eval(&ast);
    }
}
