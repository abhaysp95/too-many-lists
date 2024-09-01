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
