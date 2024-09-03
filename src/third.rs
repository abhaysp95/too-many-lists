use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
where
    T: PartialEq,
{
    pub fn new() -> List<T> {
        Self { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
            // as_deref() will work same the way it worked for Box<>
            // next: self.head.as_ref().map(|node| &**node)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// NOTE: Since we only shared access to element we can't implement IntoIter (moves) and IterMut
// (mutable ref.) for this third list as of now

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(node) = Rc::try_unwrap(node) {
                cur_link = node.next;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_prepend_tail() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_drop() {
        let list = List::new().prepend(1).prepend(2).prepend(3).prepend(4).prepend(5);
        let mut list2 = List::new();
        // calling clone() will increase strong ref count for Rc enclosing that node
        list2.head = list.head.clone().unwrap().next.clone().unwrap().next.clone();

        // confirm list 2
        let mut iter2 = list2.iter();
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        // list1 should still work
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        // let's see what dropping list2 does
        drop(list2);  // list2 is consumed here
        // but the nodes will not get dropped, because nodes for list2 has strong ref. count > 1
        // because the nodes from list1 still reference it
        // list1 should still work
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        // let's make list2 again and this time we'll drop list1 first
        // and list2 should work
        let mut list2 = List::new();
        list2.head = list.head.clone().unwrap().next.clone().unwrap().next.clone();
        drop(list);
        // confirm list 2
        let mut iter2 = list2.iter();
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);
    }
}

// NOTE: we'll come back to these
//
// pub fn prepend(&mut self, elem: T) {
//     self.head.take().map(|node| {
//         self.head = Some(Rc::new(Node {
//             elem,
//             next: Some(node),
//         }))
//     });
// }
//
// pub fn pop(&mut self) -> Option<T> {
//     self.head.take().map(|node| {
//         self.head = node.next;
//         node.elem
//     })
// }
