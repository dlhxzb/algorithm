use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::{drop_in_place, NonNull};

#[derive(Debug, Default)]
struct LinkList<T: Debug> {
    head: Option<NonNull<Node<T>>>,
}

#[derive(Debug)]
struct Node<T: Debug> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Debug> LinkList<T> {
    fn push(&mut self, data: T) {
        let s = Node {
            data,
            next: self.head.clone(),
        };
        self.head = Some(Box::leak(Box::new(s)).into());
    }

    fn pop(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            let boxed_node = Box::from_raw(node.as_ptr());
            self.head = boxed_node.next;
            boxed_node.data
        })
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Debug> Drop for LinkList<T> {
    fn drop(&mut self) {
        println!("drop list");
        if let Some(first) = self.head {
            let mut p_node = first;
            unsafe {
                while let Some(next) = p_node.as_ref().next {
                    drop(p_node.as_ptr());
                    p_node = next;
                }
                drop_in_place(p_node.as_ptr());
            }
        }
    }
}

// E0509 when an attempt is made to move out of a value whose type implements the Drop trait.
// impl<T: Debug> Drop for Node<T> {
//     fn drop(&mut self) {
//         println!("drop {:?}", self);
//     }
// }

struct Iter<'a, T: 'a + Debug> {
    current: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T: Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let res = self.current.as_ref().map(|m| &m.as_ref().data);
            self.current = self
                .current
                .as_ref()
                .map(|m| m.as_ref().next.clone())
                .flatten();
            res
        }
    }
}

#[test]
fn test_link_list() {
    let mut list = LinkList::default();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    let v = list.iter().cloned().collect::<Vec<_>>();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
    assert_eq!(list.pop(), None);
}
