use std::fmt::Display;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

impl<T> Display for Node<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.elem)
    }
}

// NOTE: trait restriction is needed because our split from matches the element with equality
// operator to break the list
impl<T> List<T> where T: PartialEq {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        // NOTE: we will not be missing all the previous nodes added to list
        // because mem::replace will return dest which was before replacement
        let node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
        // NOTE: If above is unclear, just do below
        //
        // match &self.head {
        //     None => None,
        //     Some(node) => Some(&node.elem),
        // }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
    
    /// Split on the basis of element match
    /// Returns the new list from the next node of the node which matched the elem provided as
    /// argument
    pub fn split_next(&mut self, elem: T) -> Option<List<T>> {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.elem == elem {
                let mut list = List::new();
                list.head = node.next.take();
                return Some(list);
            } 
            current = &mut node.next;
        }
        None
    }

    #[deprecated]
    /// The method needs update as this has problem of multiple mutable references. 
    /// At this point, I'm not even sure if this is doable without Rc, but we'll see in future
    pub fn split_at(&mut self, elem: T) -> Option<List<T>> {
        if let Some(ref mut node) = &mut self.head {
            if node.elem == elem {
                let mut list = List::new();
                list.head = self.head.take();
                return Some(list);
            }
            let mut node = node;
            while let Some(ref mut next_node) = node.next {
                if next_node.elem == elem {
                    let list = List::new(); 
                    // list.head = node.next.take();
                    return Some(list);
                }
                // prev = node;
                node = next_node;
            }
        }
        None
    }

    // NOTE: "pub fn merge(&mut self, list: List)" suffers from the same problem as 'split_at'
    // method ie., they both need to look ahead into next node. Split_at needs it to make new list,
    // merge will need it to know if the next.node == Link::Empty then next.node = list
}

pub struct IntoIterator<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIterator<T> {
        IntoIterator(self)
    }
}

// NOTE: We need to satisfy the trait bound for IntoIterator too because List<T> has the constraint
// This constraint can be more tight, but can't be more loose.
impl<T> Iterator for IntoIterator<T> where T: PartialEq {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// NOTE: <!-- This comment would be updated on other iter kinds of Iterator impl -->
// While this implementation works, I guess the one reason the writer showed to write it that way is
// because it shows the clear ownership transfer of the list to the iterator.
// With the below approach it is not very clear and may give sense that same list object, which was
// used to create iterator can be used again to perform list operations, which isn't true if list
// is created with into_iter() method, which will take ownership of the list object and thus will
// make the list object unusable outside of iterator
impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(ref mut boxed_node) = current {
            current = boxed_node.as_mut().next.take();
            // boxed_node gets dropped here
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;


    #[test]
    fn second_list() {
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

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(10);
        list.push(20);
        assert_eq!(list.peek(), Some(&20));
        list.peek_mut().map(|elem| {
            *elem = 30;
        });
        _ = list.pop();
        assert_eq!(list.peek_mut(), Some(&mut 10));
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(10); list.push(20); list.push(30);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push(10); list.push(20); list.push(30);

        // this into_iter() will return IntoIterator wrapper, which will take ownership of List
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }
}

