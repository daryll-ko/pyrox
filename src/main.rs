pub mod llist;

fn main() {
    let n1 = llist::Node {
        left: Box::new(llist::Nil),
        val: 2024,
        right: Box::new(llist::Nil),
    };

    if let llist::Node {
        left: _,
        val,
        right: _,
    } = n1
    {
        println!("{}", val);
    }

    if let Some(res) = n1.find(2024) {
        println!("{}", res);
    }
}
