use crate::LList::{Nil, Node};

// we need enum to define Nil (?)
enum LList {
    Node {
        left: Box<LList>,
        val: i64,
        right: Box<LList>,
    },
    Nil,
}

impl LList {
    fn find(&self, target: i64) -> Option<i64> {
        match self {
            Node {
                left: _,
                val,
                right,
            } => {
                if target == *val {
                    Some(*val)
                } else {
                    Self::find(&right, target)
                }
            }
            Nil => None,
        }
    }
}

fn main() {
    let n1 = Node {
        left: Box::new(Nil),
        val: 2024,
        right: Box::new(Nil),
    };

    if let Node {
        left: _,
        val,
        right: _,
    } = n1
    {
        println!("{}", val);
    }
}
