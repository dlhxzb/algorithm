use std::hash::Hash;

use tokio::sync::{mpsc, oneshot};

use crate::sort::lru::LRUCache;

/// Least Recently Used，缓存淘汰，无锁封装
#[derive(Clone)]
pub struct LRUCacheHandle<K, V> {
    // cache: Arc<LRUCache<K, V>>,
    tx: mpsc::UnboundedSender<(Command<K, V>, oneshot::Sender<Option<V>>)>,
}

enum Command<K, V> {
    Put(K, V),
    Get(K),
}

impl<K: Hash + Eq + Clone + std::fmt::Debug + Send + 'static, V: Clone + Send + 'static>
    LRUCacheHandle<K, V>
{
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        std::thread::spawn(move || Self::handler(capacity, rx));
        Self { tx }
    }

    pub fn get(&self, key: K) -> Option<V> {
        let (otx, orx) = oneshot::channel();
        let _ = self.tx.send((Command::Get(key), otx));
        orx.blocking_recv().unwrap()
    }

    pub fn put(&self, key: K, value: V) {
        let (otx, orx) = oneshot::channel();
        let _ = self.tx.send((Command::Put(key, value), otx));
        orx.blocking_recv().unwrap();
    }

    fn handler(
        capacity: usize,
        mut rx: mpsc::UnboundedReceiver<(Command<K, V>, oneshot::Sender<Option<V>>)>,
    ) {
        let mut cache = LRUCache::<K, V>::new_with_capacity(capacity);
        while let Some((cmd, otx)) = rx.blocking_recv() {
            match cmd {
                Command::Put(k, v) => {
                    cache.put(k, v);
                    let _ = otx.send(None);
                }
                Command::Get(k) => {
                    let _ = otx.send(cache.get(&k));
                }
            }
        }
    }
}

#[test]
fn test_lru_lockfree() {
    let handle = LRUCacheHandle::new(5);
    let handle2 = handle.clone();
    let h = std::thread::spawn(move || {
        for i in 0..5 {
            handle2.put(i, i);
        }
        handle2.get(1).unwrap();
    });
    h.join().unwrap();

    handle.put(5, 5);
    handle.put(6, 6);

    assert!(handle.get(0).is_none());
    assert!(handle.get(2).is_none());
    assert_eq!(handle.get(5).unwrap(), 5);
    assert_eq!(handle.get(6).unwrap(), 6);
}
