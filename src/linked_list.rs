use std::ptr::{NonNull};

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { next: None, prev: None, element }
    }
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        LinkedList { head: None, tail: None, len: 0 }
    }

    pub fn push_back(&mut self, elt: T) {
        let node = Box::new(Node::new(elt));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            (*node_ptr.as_ptr()).next = None;
            (*node_ptr.as_ptr()).prev = self.tail;
            let node = Some(NonNull::from(node_ptr));

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }
}