use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

pub type RcListNode<T> = Rc<RefCell<ListNode<T>>>;
type WeakListNode<T> = Weak<RefCell<ListNode<T>>>;

pub struct ListNode<T> {
    pub this : WeakListNode<T>,
    pub prev : WeakListNode<T>,
    pub next : Option<RcListNode<T>>,
    pub data : T,
}

pub struct ListNodeIterator<T> {
    head : RcListNode<T>,
    current : RcListNode<T>,
    at_end : bool,
}

impl<T> ListNode<T> {
    pub fn new(data : T) -> RcListNode<T> {
        let head = Rc::new(RefCell::new(ListNode {
            this : Weak::new(),
            prev : Weak::new(),
            next : None,
            data : data,
        }));

        head.borrow_mut().this = Rc::downgrade(&head);
        head.borrow_mut().prev = Rc::downgrade(&head);
        head.borrow_mut().next = Some(head.clone());

        head
    }

    pub fn iter(&self) -> ListNodeIterator<T> {
        ListNodeIterator {
            head : self.this.upgrade().unwrap(),
            current : self.this.upgrade().unwrap(),
            at_end : false,
        }
    }

    pub fn transfer_nodes_to_tail(&mut self, nodes_head : &RcListNode<T>) {
        let nodes_tail = nodes_head.borrow().prev.upgrade().unwrap();
        let list_prev = self.prev.upgrade().unwrap();

        self.prev = Rc::downgrade(&nodes_tail);
        nodes_tail.borrow_mut().next = Some(self.this.upgrade().unwrap());

        // If the self is a one-node list, can't re-borrow ourselves, but we are already a &mut,
        // so just make the change directly.
        if Rc::ptr_eq(&list_prev, &self.this.upgrade().unwrap()) {
            self.next = Some(Rc::clone(nodes_head));
        } else {
            list_prev.borrow_mut().next = Some(Rc::clone(nodes_head));
        }

        nodes_head.borrow_mut().prev = Rc::downgrade(&list_prev);
    }

    pub fn insert_tail(&mut self, data : T) {
        let node = ListNode::new(data);
        ListNode::transfer_nodes_to_tail(self, &node);
    }
}

impl<T> Iterator for ListNodeIterator<T> {
    type Item = RcListNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            None
        } else {
            let ret = Some(self.current.clone());

            if Rc::ptr_eq(&self.current.borrow().next.as_ref().unwrap(), &self.head) {
                self.at_end = true;
            }

            let next_current = self.current.borrow().next.as_ref().unwrap().clone();
            self.current = next_current;

            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn test_contents<T>(head : &RcListNode<T>, expected_contents : Vec<T>)
    where T : Copy + Debug + Eq {
        let actual_contents : Vec<T> = head.borrow().iter().map(|node| {
            node.borrow().data
        }).collect();

        assert_eq!(actual_contents, expected_contents);
    }

    #[test]
    fn simple_list() {
        let head = ListNode::new(0);
        for i in 1 .. 6 {
            head.borrow_mut().insert_tail(i);
        }

        test_contents(&head, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn transfer_tail() {
        let one = ListNode::new(0);
        for i in 1 .. 3 {
            one.borrow_mut().insert_tail(i);
        }

        let two = ListNode::new(100);
        for i in 101 .. 103 {
            two.borrow_mut().insert_tail(i);
        }

        one.borrow_mut().transfer_nodes_to_tail(&two);

        test_contents(&one, vec![0, 1, 2, 100, 101, 102]);
        test_contents(&two, vec![100, 101, 102, 0, 1, 2]);
    }
}
