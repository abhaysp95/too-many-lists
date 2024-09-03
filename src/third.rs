use std::rc::Rc;

pub struct List<T: std::fmt::Debug> {
    head: Link<T>,
    name: String,
}

pub type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T>
where
    T: PartialEq + std::fmt::Debug,
{
    pub fn new(name: String) -> List<T> {
        Self { head: None, name }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
            name: self.name.clone(),
        }
    }

    pub fn tail(&self) -> List<T> {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            name: self.name.clone(),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T: std::fmt::Debug> List<T> {
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

impl<T: std::fmt::Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("drop called for {}", &self.name);
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(node) = Rc::try_unwrap(node) {
                dbg!(&node.elem); // will show if you use cargo test -- --nocapture
                cur_link = node.next;
            } else {
                println!("'break' occurred. Returning...");
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::ManuallyDrop;

    use super::List;

    #[test]
    fn test_prepend_tail() {
        let list = List::new("list1".to_string());
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
        let list = List::new("list1".to_string())
            .prepend(1)
            .prepend(2)
            .prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_drop() {
        let mut list = ManuallyDrop::new(
            List::new("list1".to_string())
                .prepend(1)
                .prepend(2)
                .prepend(3)
                .prepend(4)
                .prepend(5),
        );
        let mut list2 = ManuallyDrop::new(List::new("list2".to_string()));
        // calling clone() will increase strong ref count for Rc enclosing that node
        list2.head = list
            .head
            .clone()
            .unwrap()
            .next
            .clone()
            .unwrap()
            .next
            .clone();

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
        println!("Dropping list2");
        // drop(list2);  // list2 is consumed here
        unsafe {
            ManuallyDrop::drop(&mut list2);
        }
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
        let mut list2 = List::new("list2R".to_string());
        list2.head = list
            .head
            .clone()
            .unwrap()
            .next
            .clone()
            .unwrap()
            .next
            .clone();
        dbg!("Dropping list");
        unsafe {
            ManuallyDrop::drop(&mut list);
        }
        // confirm list 2
        let mut iter2 = list2.iter();
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        // NOTE: You'll see bunch of following sentence at the start upon running this test
        //  - drop called for list1
        //  - 'break' occurred. Returning...
        // This is because when we prepend(...) to list, it is consuming old list, returning new
        // list and just before returning dropping old list (but not dropping any nodes).
        // Then you'll see the actuall drop for node happens
    }

    // NOTE: to show output/debug output for tests, pass:
    // --nocapture (disables the hiding of stdout for successful tests)
    // BONUS: --color always (keeps the colors from test)
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
