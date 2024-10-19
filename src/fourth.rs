use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // NOTE: if it just would've been Rc, you wouldn't have been able to mutate it for
                // old_head.prev or for new_node.next
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(old_node) => {
                old_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_node);
                self.tail = Some(new_node);
            },
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            } else {
                self.tail.take();
            }
            Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(old_tail) = self.tail.take() {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }
            Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem)
        } else {
            None
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            // map can be used on Ref too
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            // map can be used on Ref too
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        // Ref doesn't implement comparision, so no eq!
        assert!(list.peek_front().is_none());
        
        list.push_front(1); list.push_front(2); list.push_front(3);
        assert_eq!(*list.peek_front().unwrap(), 3);
        list.pop_front();
        list.pop_front();

        // check exhaustion
        assert_eq!(*list.peek_front().unwrap(), 1);
        list.pop_front();
        assert!(list.peek_front().is_none());
    }
}
