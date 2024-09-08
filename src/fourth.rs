use std::{cell::RefCell, rc::Rc};

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
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
            },
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }
}
