use std::ptr::NonNull;
use std::marker::PhantomData;


pub struct LinkedListNode<T> {
    data: T,
    next: Option<NonNull<LinkedListNode<T>>>,
    prev: Option<NonNull<LinkedListNode<T>>>,
    _boo: PhantomData<LinkedListNode<T>>,
}
impl<T> LinkedListNode<T> {
    fn new(data: T) -> NonNull<Self> {
        let node = LinkedListNode {
            data,
            next: None,
            prev: None,
            _boo: Default::default(),
        };
        let node_bx = Box::new(node);
        let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node_bx)) };
        node_ptr
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    length: usize,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn push(&mut self, data: T) {
        let mut node_ptr = LinkedListNode::new(data);
        match self.tail {
            Some(mut tail) => {
                unsafe {
                    tail.as_mut().next = Some(node_ptr);
                    node_ptr.as_mut().prev = Some(tail);
                }
                self.tail = Some(node_ptr);
            }
            None => {
                self.head = Some(node_ptr);
                self.tail = Some(node_ptr);
            }
        }
        self.length += 1;
    }
    pub fn push_head(&mut self, data: T) {
        let mut node_ptr = LinkedListNode::new(data);
        match self.head {
            Some(mut head) => {
                unsafe {
                    head.as_mut().prev = Some(node_ptr);
                    node_ptr.as_mut().next = Some(head);
                }
                self.head = Some(node_ptr);
                self.length += 1;
            }
            None => {
                self.head = Some(node_ptr);
                self.tail = Some(node_ptr);
                self.length += 1;
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.tail {
            Some(tail) => {
                // len != 0
                match unsafe { tail.as_ref().prev } {
                    Some(mut tprev) => {
                        // len > 1
                        unsafe {
                            tprev.as_mut().next = None;

                            // tail still has a pointer to tprev, but it won't matter once we release it
                            let tail_bx = Box::from_raw(tail.as_ptr());
                            // tail_bx will go out of scope and therefore be deallocated

                            self.tail = Some(tprev);
                            self.length -= 1;

                            Some(tail_bx.data)
                        }                        
                    }
                    None => {
                        // len == 1
                        self.head = None;
                        self.tail = None;
                        unsafe {
                            let tail_bx = Box::from_raw(tail.as_ptr());
                            // tail_bx will go out of scope and therefore be deallocated

                            self.length -= 1;

                            Some(tail_bx.data)
                        }
                    }
                }
            }
            // len == 0
            None => None,
        }
    }
    pub fn pop_head(&mut self) -> Option<T> {
        match self.head {
            Some(head) => {
                // len != 0
                match unsafe { head.as_ref().next } {
                    Some(mut hnext) => {
                        // len > 1
                        unsafe {
                            hnext.as_mut().next = None;

                            // head still has a pointer to hnext, but it won't matter once we release it
                            let head_bx = Box::from_raw(head.as_ptr());
                            // head_bx will go out of scope and therefore be deallocated

                            self.head = Some(hnext);
                            self.length -= 1;

                            Some(head_bx.data)
                        }
                    }
                    None => {
                        // len == 1
                        self.head = None;
                        self.tail = None;
                        unsafe {
                            let head_bx = Box::from_raw(head.as_ptr());
                            // head_bx will go out of scope and therefore be deallocated

                            self.length -= 1;

                            Some(head_bx.data)
                        }
                    }
                }
            }
            // len == 0
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn new_creates_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_and_len() {
        let mut list = LinkedList::new();
        list.push(1);
        assert_eq!(list.len(), 1);
        list.push(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn push_head_and_len() {
        let mut list = LinkedList::new();
        list.push_head(1);
        assert_eq!(list.len(), 1);
        list.push_head(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_non_empty_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_head_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn pop_head_non_empty_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), Some(2));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn mixed_operations() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push_head(0);
        assert_eq!(list.pop_head(), Some(0));
        assert_eq!(list.len(), 2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.len(), 0);
    }
}

