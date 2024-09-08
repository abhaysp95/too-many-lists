use std::rc::Rc;

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>
}

pub type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            prev: None,
            next: None,
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}
