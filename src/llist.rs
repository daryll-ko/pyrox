pub use self::LList::{Nil, Node};

pub enum LList {
    Node {
        left: Box<LList>,
        val: i64,
        right: Box<LList>,
    },
    Nil,
}

impl LList {
    pub fn find(&self, target: i64) -> Option<i64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_works_on_empty_list() {
        let l = Nil;
        assert_eq!(l.find(0), None);
    }

    #[test]
    fn find_works_on_list_with_one_element() {
        let l = Node {
            left: Box::new(Nil),
            val: 5,
            right: Box::new(Nil),
        };
        assert_eq!(l.find(0), None);
        assert_eq!(l.find(5), Some(5));
    }
}
