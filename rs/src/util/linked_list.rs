#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn disp_list(mut l: &Option<Box<ListNode>>) {
    let mut v = vec![];
    while let Some(x) = l {
        v.push(x.val);
        l = &x.next;
    }
    println!("{:?}", v);
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

// utils
type ListLink = Option<Box<ListNode>>;
fn len(head: &ListLink) -> usize {
    let mut len = 0;
    let mut curr = head.as_ref();
    while let Some(node) = curr.take() {
        curr = node.next.as_ref();
        len += 1;
    }
    len
}

// foreach node in list, append it next to dummy
fn reverse(mut head: ListLink) -> ListLink {
    let mut dummy = ListNode::new(0);
    while let Some(mut node) = head.take() {
        // 1. head to next
        head = node.next.take();
        // 2. dummy /cut/ node -> dummy.next
        node.next = dummy.next.take();
        // 3. dummy -> node
        dummy.next = Some(node);
    }
    dummy.next
}

fn has_next(head: &ListLink) -> bool {
    head.as_ref().map_or(false, |n| n.next.is_some())
}

// iterator?
// if len >= k, return kth(0-index), else return tail
fn kth_or_tail(head: &ListLink, k: usize) -> Result<&ListLink, &ListLink> {
    assert!(head.is_some());

    if k == 0 {
        return Ok(head);
    }

    let mut kk = 0;
    let mut prev = head;
    while has_next(prev) {
        prev = &prev.as_ref().unwrap().next;
        kk += 1;
        if kk == k { break; }
    }
    assert!(prev.is_some());
    if kk == k {
        return Ok(prev)
    } else {
        return Err(prev)
    }
}

fn kth_or_tail_mut(head: &mut ListLink, k: usize) -> Result<&mut ListLink, &mut ListLink> {
    assert!(head.is_some());

    if k == 0 {
        return Ok(head);
    }

    let mut kk = 0;
    let mut prev = head;
    while has_next(prev) {
        prev = &mut prev.as_mut().unwrap().next;
        kk += 1;
        if kk == k { break; }
    }
    assert!(prev.is_some());
    if kk == k {
        return Ok(prev)
    } else {
        return Err(prev)
    }
}

fn tail(head: &ListLink) -> &ListLink {
    assert!(head.is_some());

    let mut prev = head;
    while has_next(prev) {
        prev = &prev.as_ref().unwrap().next;
    }
    assert!(prev.is_some());
    prev
}

fn tail_mut(head: &mut ListLink) -> &mut ListLink {
    assert!(head.is_some());

    let mut prev = head;
    while has_next(prev) {
        prev = &mut prev.as_mut().unwrap().next;
    }
    assert!(prev.is_some());
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth() {
        let l = linked![];
        let k = kth_or_tail(&l, 0);
        assert!(k.is_err());

        let l = linked![];
        let k = kth_or_tail(&l, 2);
        assert!(k.is_err());

        let l = linked![1];
        let k = kth_or_tail(&l, 1);
        assert!(k.is_err());
        println!("{:?}", k);

        let l = linked![1];
        let k = kth_or_tail(&l, 0);
        assert!(k.is_ok());
        println!("{:?}", k);

        let l = linked![1,2,3];
        let k = kth_or_tail(&l, 1);
        assert!(k.is_ok());
        println!("{:?}", k);

        let l = linked![1,2,3];
        let k = kth_or_tail(&l, 2);
        assert!(k.is_ok());
        println!("{:?}", k);

        let l = linked![1,2,3];
        let k = kth_or_tail(&l, 3);
        assert!(k.is_err());
        println!("{:?}", k);
    }

    #[test]
    fn test_kth_mut() {
        // k <= len, ok, append
        let mut l = linked![1];
        let k = kth_or_tail_mut(&mut l, 0).unwrap();
        assert!(k.is_some());
        k.as_mut().map(|n| n.val = 2);
        k.as_mut().map(|n| n.next = Some(Box::new(ListNode::new(0))));
        assert_eq!(2, len(&l));
        disp_list(&l);

        // k > len, err, append
        let mut l = linked![1,2,3];
        let k = kth_or_tail_mut(&mut l, 3).unwrap_err();
        assert!(k.is_some());
        k.as_mut().map(|n| n.val = -1);
        k.as_mut().map(|n| n.next = Some(Box::new(ListNode::new(0))));
        assert_eq!(4, len(&l));
        disp_list(&l); // 1 2 -1 3


        // k < len, ok, insert at middle
        let mut l = linked![1,2,3];
        let k = kth_or_tail_mut(&mut l, 1).unwrap();
        assert!(k.is_some());
        k.as_mut().map(|n| n.val = -1);
        k.as_mut().map(|n| {
            let mut new_node = Box::new(ListNode::new(0));
            new_node.next = n.next.take();
            n.next = Some(new_node);
        });
        assert_eq!(4, len(&l));
        disp_list(&l); // 1 -1 0 3
    }
}