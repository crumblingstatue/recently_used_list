#![warn(missing_docs)]

//! A simple recently-used list with serde support

use serde::{Deserialize, Serialize};

/// Recently used list
#[derive(Serialize, Deserialize, Debug)]
pub struct RecentlyUsedList<T> {
    items: Vec<T>,
    #[serde(default = "default_capacity")]
    capacity: usize,
}

impl<T> Default for RecentlyUsedList<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: default_capacity(),
        }
    }
}

const fn default_capacity() -> usize {
    7
}

impl<T: PartialEq> RecentlyUsedList<T> {
    /// Add `item` to the list as the most recently used item.
    pub fn use_(&mut self, item: T) {
        let pos = self.items.iter().position(|it| it == &item);
        if let Some(pos) = pos {
            self.items.remove(pos);
        }
        self.items.push(item);
        if self.items.len() > self.capacity {
            self.items.remove(0);
        }
    }
    /// Find and remove `item` from the list.
    ///
    /// Does nothing if `item` is not found in the list.
    pub fn find_remove(&mut self, item: T) {
        let pos = self.items.iter().position(|it| it == &item);
        if let Some(pos) = pos {
            self.items.remove(pos);
        }
    }
}

/// Iterator over recently used list elements, in order of most to least recent
///
/// ## Implementation details
///
/// Unfortunately we can't return `impl Iterator` from [`RecentlyUsedList::iter`], because it
/// slightly alters borrow checking behavior due to drop glue. This concrete type doesn't have
/// any drop glue, but `impl Iterator` doesn't know that.
pub type Iter<'a, T> = std::iter::Rev<std::slice::Iter<'a, T>>;

impl<T> RecentlyUsedList<T> {
    /// Creates a new recently used list with capacity specified by `cap`,
    /// instead of the default (7).
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: cap,
        }
    }
    /// Returns the most recent item, if the list is not empty
    pub fn most_recent(&self) -> Option<&T> {
        self.items.last()
    }
    /// Returns the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Whether this list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns an iterator over the list items, in order of most to least recent
    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter().rev()
    }
    /// Returns the capacity of this list
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Sets the capacity of this list
    pub fn set_capacity(&mut self, cap: usize) {
        self.capacity = cap;
    }
    /// Similar to `Vec::retain`, but iterates in recent-to-least-recent order.
    pub fn retain<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) {
        if self.items.is_empty() {
            return;
        }
        let mut idx = self.items.len() - 1;
        loop {
            if !f(&mut self.items[idx]) {
                self.items.remove(idx);
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
    }
    /// Clear all items from this list
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[test]
fn test() {
    let mut ru = RecentlyUsedList::default();
    ru.use_(4);
    ru.use_(8);
    assert_eq!(ru.most_recent(), Some(&8));
    ru.use_(4);
    assert_eq!(ru.most_recent(), Some(&4));
    assert_eq!(ru.len(), 2);
    for i in 0..10 {
        ru.use_(i);
    }
    assert_eq!(ru.len(), 7);
    assert_eq!(ru.most_recent(), Some(&9));
    let items: Vec<_> = ru.iter().collect();
    assert_eq!(items[0], &9);
    assert_eq!(items[6], &3);
}
