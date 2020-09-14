use crossbeam::epoch::{Atomic, Guard, Shared};
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering;

/// Entry in a bin
///
/// will _generally_ be `Node`. Any entry that is not first in the bin, will be a `Node`.
// pub(crate) makes an item visible within the current crate.
pub(crate) enum BinEntry<K, V> {
    Node(Node<K, V>),
}

impl<K, V> BinEntry<K, V>
where
    K: Eq,
{
    pub(crate) fn find<'g>(
        &'g self,
        hash: u64,
        key: &K,
        guard: &'g Guard,
    ) -> Option<Shared<'g, Node<K, V>>> {
        match *self {
            BinEntry::Node(ref n) => {
                if n.hash == hash && &n.key == key {
                    return Some(Shared::from(n as *const _));
                }
                let next = n.next.load(Ordering::Acquire, guard);
                if next.is_null() {
                    return None;
                }
                return Some(next);
            }
        }
    }
}

pub(crate) struct Node<K, V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>, // To get a mutable pointer even though there are any mutable pointer (unsafe)
    pub(crate) next: Atomic<Node<K, V>>,
}
