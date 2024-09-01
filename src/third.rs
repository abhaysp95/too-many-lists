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
