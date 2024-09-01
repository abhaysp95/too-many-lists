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

    // NOTE: why split_at() is written this way ? You'll get to know once you reach second
    // list(second.rs)
    pub fn split_at(&mut self, elem: i32) -> Option<List> {
        let mut next_node = &mut self.head;
        loop {
            match next_node {
                Link::More(node) if node.elem == elem => {
                    let mut list = List::new();
                    list.head = mem::replace(&mut next_node, Link::Empty);
                    break Some(list);
                },
                Link::More(node) => next_node = &mut node.next,
                Link::Empty => break None,
            }
        }
    }

    pub fn merge(&mut self, mut list: List) {
        match self.head {
            Link::Empty => {
                self.head = mem::replace(&mut list.head, Link::Empty);
            },
            Link::More(_) => {
                let mut next_node = &mut self.head;
                while let Link::More(node) = next_node {
                    match node.next {
                        Link::Empty => {
                            node.next = mem::replace(&mut list.head, Link::Empty);
                            break;
                        },
                        Link::More(_) => next_node = &mut node.next,
                    }
                }
            }
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

    #[test]
    fn test_split_next() {
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

    #[test]
    fn test_split_at() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        let no_list = list.split_at(10);
        assert!(no_list.is_none());

        let list2 = list.split_at(1);
        assert!(list2.is_some());
        let mut list2 = list2.unwrap();
        assert_eq!(list2.pop(), Some(1));
        assert_eq!(list2.pop(), None);

        // move after first element
        let moved_list = list.split_at(5);
        assert!(moved_list.is_some());
        // old list should have no element now
        assert_eq!(list.pop(), None);

        // break from between
        let mut moved_list = moved_list.unwrap();
        let mut half_list = moved_list.split_at(3);
        assert!(half_list.is_some());
        // exhaust both the list now
        assert_eq!(moved_list.pop(), Some(5));
        assert_eq!(moved_list.pop(), Some(4));
        assert_eq!(moved_list.pop(), None);
        assert_eq!(half_list.as_mut().unwrap().pop(), Some(3));
        assert_eq!(half_list.as_mut().unwrap().pop(), Some(2));
        assert_eq!(half_list.as_mut().unwrap().pop(), None);
    }

    #[test]
    fn test_merge() {
        let mut list1 = List::new();
        let mut list2 = List::new();

        // when both the lists are empty
        list1.merge(list2);
        assert_eq!(list1.pop(), None);

        // when a list is empty
        list2 = List::new();
        list2.push(4);
        list2.push(5);
        list1.merge(list2);
        assert_eq!(list1.pop(), Some(5));
        assert_eq!(list1.pop(), Some(4));
        assert_eq!(list1.pop(), None);

        // when there's only one element in first list
        list1.push(1);
        list2 = List::new();
        list2.push(4);
        list2.push(5);
        list1.merge(list2);
        assert_eq!(list1.pop(), Some(1));
        assert_eq!(list1.pop(), Some(5));
        assert_eq!(list1.pop(), Some(4));
        assert_eq!(list1.pop(), None);

        // when there are more elements
        list1.push(1);
        list1.push(2);
        list1.push(3);
        list2 = List::new();
        list2.push(4);
        list2.push(5);
        list2.merge(list1);

        assert_eq!(list2.pop(), Some(5));
        assert_eq!(list2.pop(), Some(4));
        assert_eq!(list2.pop(), Some(3));
        assert_eq!(list2.pop(), Some(2));
        assert_eq!(list2.pop(), Some(1));
        assert_eq!(list2.pop(), None);
    }
}
