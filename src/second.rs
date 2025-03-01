use std::fmt::Display;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.elem)
    }
}

// NOTE: trait restriction is needed because our split from matches the element with equality
// operator to break the list
impl<T> List<T>
where
    T: PartialEq,
{
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
        self.head.as_ref().map(|node| &node.elem)
        // NOTE: If above is unclear, just do below
        //
        // match &self.head {
        //     None => None,
        //     Some(node) => Some(&node.elem),
        // }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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

    /// **WARNING**: This method is for reference/learning purpose only and not to be used as this has
    /// problem
    pub fn early_split_at(&mut self, elem: T) -> Option<List<T>> {
        if let Some(ref mut node) = &mut self.head {
            if node.elem == elem {
                let mut list = List::new();
                list.head = self.head.take();
                return Some(list);
            } else {
                let mut node_next = &mut node.next;
                // While let below is borrowing node_next for the whole block, disregarding the
                // control-flow (return statement we have inside if)
                // I got to know that this is known limitation from borrow-checker
                while let Some(ref mut node) = node_next {
                    if node.elem == elem {
                        #[allow(unused_mut)]
                        let mut list = List::new();
                        // list.head = node_next.take();
                        return Some(list);
                    }
                    node_next = &mut node.next;
                }
            }
        }
        None
    }

    pub fn split_at(&mut self, elem: T) -> Option<List<T>> {
        let mut node_next = &mut self.head;
        loop {
            match node_next {
                Some(node) if node.elem == elem => {
                    let mut list = List::new();
                    list.head = node_next.take();
                    break Some(list);
                }
                Some(node) => node_next = &mut node.next,
                None => break None,
            };
        } // <- this is expression
          // and thus when breaked with some value from loop it'll return that value from here for
          // this method
    }

    pub fn merge(&mut self, mut list: List<T>) {
        match &mut self.head {
            None => {
                self.head = list.head.take();
                return;
            },
            Some(_) => {
                let mut next_node = &mut self.head;
                while let Some(ref mut node) = next_node {
                    if node.next.is_none() {
                        node.next = list.head.take();
                        break;
                    }
                    next_node = &mut node.next;
                }
            },
        };
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// NOTE: We need to satisfy the trait bound for IntoIterator too because List<T> has the constraint
// This constraint can be more tight, but can't be more loose.
impl<T> Iterator for IntoIter<T>
where
    T: PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // self.next = node.next.as_ref().map(|node| node.as_ref());
            // self.next = node.next.as_ref().map(|node| &**node);
            //
            // NOTE: both the above lines are correct, here as_deref() is deref coercion for
            // &**node
            //
            // self.next = node.next.as_deref();
            // or
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
            // NOTE: above is short (or better syntactically) for
            // next: self.head.as_mut().map(|node| {
            //     &mut **node
            // })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // self.next = node.next.as_mut().map(|node| node.as_mut());
            // self.next = node.next.as_mut().map(|node| &mut **node);
            //
            // NOTE: both the above lines are correct, here as_deref() is deref coercion for
            // &**node
            //
            // self.next = node.next.as_deref_mut();
            // or
            self.next = node.next.as_mut().map::<&mut Node<T>, _>(|node| node);
            &mut node.elem
        })
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    // NOTE: <!-- This comment would be updated on other iter kinds of Iterator impl -->
    // While this implementation works, I guess the one reason the writer showed to write it that way is
    // because it shows the clear ownership transfer of the list to the iterator.
    // With the below approach it is not very clear and may give sense that same list object, which was
    // used to create iterator can be used again to perform list operations, which isn't true if list
    // is created with into_iter() method, which will take ownership of the list object and thus will
    // make the list object unusable outside of iterator
    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // NOTE: iter() can't be done directly because traits can't hold any value on their own, and we
    // need some cursor (variable) to tell us where iterator is pointing at some particular time
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
        assert!(list2.is_some()); // we got the list
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
    fn test_into_iter() {
        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);

        // this into_iter() will return IntoIterator wrapper, which will take ownership of List
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 30));
        assert_eq!(iter.next(), Some(&mut 20));
        assert_eq!(iter.next(), Some(&mut 10));
        assert_eq!(iter.next(), None);
    }
}
