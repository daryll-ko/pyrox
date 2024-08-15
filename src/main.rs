use crate::LList::{Nil, Node};

// we need enum to define Nil (?)
enum LList {
    Node(Box<LList>, i64, Box<LList>),
    Nil,
}

fn get_value(l: LList) -> i64 {
    match l {
        Node(_, val, _) => val,
        Nil => 0,
    }
}

fn main() {
    let n1 = Node(Box::new(Nil), 2024, Box::new(Nil));
    println!("{}", get_value(n1));
}
