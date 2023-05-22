use std::collections::HashMap;
use std::hash::Hash;
use std::ptr::NonNull;

/// Least Recently Used，缓存淘汰
pub struct LRUCache<K, V> {
    cache: HashMap<K, Node<K, V>>,
    head: NonNull<Node<K, V>>, // 为节省空间只定义为指针
    tail: NonNull<Node<K, V>>,
    capacity: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    prev: NonNull<Node<K, V>>,
    next: NonNull<Node<K, V>>,
}

impl<K: Hash + Eq + Clone + std::fmt::Debug, V: Clone> LRUCache<K, V> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            cache: HashMap::with_capacity(capacity),
            head: NonNull::dangling(),
            tail: NonNull::dangling(),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let node = if let Some(node) = self.cache.get_mut(&key) {
            node as *mut _
        } else {
            // 缓存已满，删除tail
            if self.cache.len() >= self.capacity {
                // SAFETY: cache非空时，tail指针已被初始化
                let key = unsafe { &self.tail.as_ref().key };
                self.remove_link(self.tail.as_ptr());
                self.cache.remove(key).unwrap();
            }

            let node = Node {
                key: key.clone(),
                value,
                prev: NonNull::dangling(),
                next: NonNull::dangling(),
            };
            self.cache.insert(key.clone(), node);
            self.cache.get_mut(&key).unwrap() as *mut _
        };

        self.push_front_link(node);
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.cache.get_mut(key) {
            let node = node as *mut _;
            self.activate(node);
            unsafe { Some((*node).value.clone()) }
        } else {
            None
        }
    }

    // 激活node到最前
    fn activate(&mut self, node_ptr: *mut Node<K, V>) {
        let node = NonNull::new(node_ptr).unwrap();
        if self.head == node {
            return;
        }
        self.remove_link(node_ptr);
        self.push_front_link(node_ptr);
    }

    // 在链表中删除。修改前节点next，后节点prev，若首节点修改head，若尾节点修改tail
    fn remove_link(&mut self, node_ptr: *mut Node<K, V>) {
        let node = NonNull::new(node_ptr).unwrap();
        // SAFTY: node都是从HashMap里查出来的，必不为空
        unsafe {
            // 后节点prev
            if (*node_ptr).next != NonNull::dangling() {
                (*node_ptr).next.as_mut().prev = (*node_ptr).prev;
            }
            // 前节点next
            if (*node_ptr).prev != NonNull::dangling() {
                (*node_ptr).prev.as_mut().next = (*node_ptr).next;
            }
            // 如果该node是tail，修改tail
            if self.tail == node {
                self.tail = (*node_ptr).prev;
            }
        };
    }

    // 插入到单链表头。修改自身prev/next，head，后节点prev，若尾节点修改tail
    fn push_front_link(&mut self, node_ptr: *mut Node<K, V>) {
        let node = NonNull::new(node_ptr).unwrap();
        // SAFTY: node都是从HashMap里查出来的
        unsafe {
            // 自身next，由于已在队首prev dangling即可
            (*node_ptr).next = self.head;
            (*node_ptr).prev = NonNull::dangling();
            // 插入第一个元素时初始化tail
            if self.tail == NonNull::dangling() {
                self.tail = node;
            } else {
                // 后节点prev
                self.head.as_mut().prev = node;
            }
            self.head = node;
        };
    }

    pub fn show_keys(&self) -> Vec<K>
    where
        K: std::fmt::Debug,
    {
        let mut v = vec![];
        let mut p = self.head;
        while p != NonNull::dangling() {
            unsafe {
                v.push(p.as_ref().key.clone());
                p = p.as_ref().next;
            }
        }
        v
    }
}

#[test]
fn test_lru() {
    let mut cache = LRUCache::new_with_capacity(5);
    for i in 0..5 {
        cache.put(i, i);
    }
    cache.get(&1).unwrap();
    assert_eq!(cache.show_keys(), [1, 4, 3, 2, 0]);

    cache.put(5, 5);
    cache.put(6, 6);
    assert_eq!(cache.show_keys(), [6, 5, 1, 4, 3,]);

    assert!(cache.get(&0).is_none());
    assert!(cache.get(&2).is_none());
    assert_eq!(cache.get(&5).unwrap(), 5);
    assert_eq!(cache.get(&6).unwrap(), 6);
}
