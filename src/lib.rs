use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentlyUsedList<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> Default for RecentlyUsedList<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 7,
        }
    }
}

impl<T: PartialEq> RecentlyUsedList<T> {
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

    pub fn remove(&mut self, item: T) {
        let pos = self.items.iter().position(|it| it == &item);
        if let Some(pos) = pos {
            self.items.remove(pos);
        }
    }
}

impl<T> RecentlyUsedList<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity: cap,
        }
    }
    pub fn most_recent(&self) -> Option<&T> {
        self.items.last()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().rev()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
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
