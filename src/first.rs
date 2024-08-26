use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // NOTE: we will not be missing all the previous nodes added to list
        // because mem::replace will return dest which was before replacement
        let node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let result = node.elem;
                self.head = node.next;
                Some(result)
            }
        }
    }
    
    pub fn merge(&mut self, list: List) {
        match self.head {
            Link::Empty => {},
            Link::More(_) => {},
        }

    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(ref mut boxed_node) = current {
            current = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node gets dropped here
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;


    #[test]
    fn first_list() {
        let mut list = List::new();

        // check if list empty
        assert_eq!(list.pop(), None);

        // insert into list
        list.push(1);
        list.push(2);
        list.push(3);

        // check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // is something corrupted
        list.push(4);
        list.push(5);

        // check removal again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
