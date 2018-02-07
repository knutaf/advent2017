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

pub struct ListSegment<T> {
    start : RcListNode<T>,
    end : RcListNode<T>,
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
        head.borrow_mut().next = Some(Rc::clone(&head));

        head
    }

    pub fn iter(&self) -> ListNodeIterator<T> {
        ListNodeIterator {
            head : self.this.upgrade().unwrap(),
            current : self.this.upgrade().unwrap(),
            at_end : false,
        }
    }

    pub fn transfer_nodes_before(&mut self, nodes_head : &RcListNode<T>) {
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

    pub fn insert_before(&mut self, data : T) {
        let node = ListNode::new(data);
        self.transfer_nodes_before(&node);
    }

    pub fn transfer_nodes_after(&mut self, nodes_head : &RcListNode<T>) {
        let nodes_tail = nodes_head.borrow().prev.clone();
        let list_next = Rc::clone(self.next.as_ref().unwrap());

        // If the self is a one-node list, can't re-borrow ourselves, but we are already a &mut,
        // so just make the change directly.
        if Rc::ptr_eq(&list_next, &self.this.upgrade().unwrap()) {
            self.prev = nodes_tail.clone();
        } else {
            list_next.borrow_mut().prev = nodes_tail.clone();
        }

        nodes_tail.upgrade().unwrap().borrow_mut().next = Some(Rc::clone(&list_next));

        self.next = Some(Rc::clone(&nodes_head));
        nodes_head.borrow_mut().prev = self.this.clone();
    }

    pub fn insert_after(&mut self, data : T) {
        let node = ListNode::new(data);
        self.transfer_nodes_after(&node);
    }

    pub fn create_segment(&self, length : usize) -> ListSegment<T> {
        ListSegment {
            start : self.this.upgrade().unwrap(),
            end : self.iter().take(length).last().unwrap(),
        }
    }
}

impl<T> Iterator for ListNodeIterator<T> {
    type Item = RcListNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            None
        } else {
            let ret = Some(Rc::clone(&self.current));

            if Rc::ptr_eq(&self.current.borrow().next.as_ref().unwrap(), &self.head) {
                self.at_end = true;
            }

            let next_current = Rc::clone(self.current.borrow().next.as_ref().unwrap());
            self.current = next_current;

            ret
        }
    }
}

impl<T> ListSegment<T> {
    // note: consumes self as it splits the list and extracts this into a standalone list
    fn extract(self) -> (RcListNode<T>, RcListNode<T>) {
        if Rc::ptr_eq(&self.start, &self.end) && Rc::ptr_eq(&self.start, self.start.borrow().next.as_ref().unwrap()) {
            (self.start, self.end)
        } else {
            let remainder = Rc::clone(self.end.borrow().next.as_ref().unwrap());
            self.start.borrow().prev.upgrade().unwrap().borrow_mut().next = Some(Rc::clone(&remainder));

            let start_prev = self.start.borrow().prev.clone();
            self.end.borrow().next.as_ref().unwrap().borrow_mut().prev = start_prev;

            self.start.borrow_mut().prev = Rc::downgrade(&self.end);
            self.end.borrow_mut().next = Some(Rc::clone(&self.start));

            (self.start, remainder)
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
            head.borrow_mut().insert_before(i);
        }

        test_contents(&head, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn transfer_before() {
        let one = ListNode::new(0);
        for i in 1 .. 3 {
            one.borrow_mut().insert_before(i);
        }

        let two = ListNode::new(100);
        for i in 101 .. 103 {
            two.borrow_mut().insert_before(i);
        }

        one.borrow_mut().transfer_nodes_before(&two);

        test_contents(&one, vec![0, 1, 2, 100, 101, 102]);
        test_contents(&two, vec![100, 101, 102, 0, 1, 2]);
    }

    #[test]
    fn insert_after() {
        let head = ListNode::new(0);
        for i in 1 .. 6 {
            head.borrow_mut().insert_after(i);
        }

        test_contents(&head, vec![0, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn extract_one_from_many() {
        let ls = ListNode::new(0);
        for i in 1 .. 6 {
            ls.borrow_mut().insert_before(i);
        }

        let segment_in_list = ls.borrow().create_segment(1);
        let (segment_standalone, ls) = segment_in_list.extract();

        test_contents(&ls, vec![1, 2, 3, 4, 5]);
        test_contents(&segment_standalone, vec![0]);
    }

    #[test]
    fn extract_many() {
        let ls = ListNode::new(0);
        for i in 1 .. 6 {
            ls.borrow_mut().insert_before(i);
        }

        let segment_in_list = ls.borrow().create_segment(3);
        let (segment_standalone, ls) = segment_in_list.extract();

        test_contents(&ls, vec![3, 4, 5]);
        test_contents(&segment_standalone, vec![0, 1, 2]);
    }

    #[test]
    fn extract_all_from_one() {
        let ls = ListNode::new(0);

        let segment_in_list = ls.borrow().create_segment(1);
        let (segment_standalone, ls) = segment_in_list.extract();

        test_contents(&ls, vec![0]);
        test_contents(&segment_standalone, vec![0]);
    }

    #[test]
    fn extract_all_from_all() {
        let ls = ListNode::new(0);
        for i in 1 .. 3 {
            ls.borrow_mut().insert_before(i);
        }

        let segment_in_list = ls.borrow().create_segment(3);
        let (segment_standalone, ls) = segment_in_list.extract();

        test_contents(&ls, vec![0, 1, 2]);
        test_contents(&segment_standalone, vec![0, 1, 2]);
    }

    #[test]
    fn extract_too_many() {
        let ls = ListNode::new(0);
        for i in 1 .. 3 {
            ls.borrow_mut().insert_before(i);
        }

        let segment_in_list = ls.borrow().create_segment(4);
        let (segment_standalone, ls) = segment_in_list.extract();

        test_contents(&ls, vec![0, 1, 2]);
        test_contents(&segment_standalone, vec![0, 1, 2]);
    }
}
