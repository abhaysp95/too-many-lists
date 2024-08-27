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
    
    /// Split on the basis of element match
    /// Returns the new list from the next node of the node which matched the elem provided as
    /// argument
    pub fn split_next(&mut self, elem: i32) -> Option<List> {
        let mut current = &mut self.head;
        while let Link::More(ref mut node) = current {
            if node.elem == elem {
                let mut list = List::new();
                list.head = mem::replace(&mut node.next, Link::Empty);
                return Some(list);
            } 
            current = &mut node.next;
        }
        None
    }

    #[deprecated]
    /// The method needs update as this has problem of multiple mutable references. 
    /// At this point, I'm not even sure if this is doable without Rc, but we'll see in future
    pub fn split_at(&mut self, elem: i32) -> Option<List> {
        if let Link::More(ref mut node) = &mut self.head {
            if node.elem == elem {
                let mut list = List::new();
                list.head = mem::replace(&mut self.head, Link::Empty);
                return Some(list);
            }
            let mut node = node;
            while let Link::More(ref mut next_node) = node.next {
                if next_node.elem == elem {
                    let list = List::new(); 
                    // list.head = mem::replace(&mut node.next, Link::Empty);
                    return Some(list);
                }
                node = next_node;
            }
        }
        None
    }

    // NOTE: "pub fn merge(&mut self, list: List)" suffers from the same problem as 'split_at'
    // method ie., they both need to look ahead into next node. Split_at needs it to make new list,
    // merge will need it to know if the next.node == Link::Empty then next.node = list
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

    #[test]
    fn test_split() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        let no_list = list.split_next(10);
        assert!(no_list.is_none());

        let list2 = list.split_next(1);
        assert!(list2.is_some());  // we got the list
        // but it shouldn't have any element
        let mut list2 = list2.unwrap(); 
        assert_eq!(list2.pop(), None);

        // move after first element
        let moved_list = list.split_next(5);
        assert!(moved_list.is_some());
        // old list should have one element now
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // break from between
        let mut moved_list = moved_list.unwrap();
        let mut half_list = moved_list.split_next(3);
        assert!(half_list.is_some());
        // exhaust both the list now
        assert_eq!(moved_list.pop(), Some(4));
        assert_eq!(moved_list.pop(), Some(3));
        assert_eq!(moved_list.pop(), None);
        assert_eq!(half_list.as_mut().unwrap().pop(), Some(2));
        assert_eq!(half_list.as_mut().unwrap().pop(), Some(1));
        assert_eq!(half_list.as_mut().unwrap().pop(), None);
    }
}
