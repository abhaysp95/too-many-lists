use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

pub type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> where T: PartialEq {
    pub fn new() -> List<T> {
        Self {
            head: None
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
