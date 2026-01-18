#![allow(clippy::nursery)]
#![allow(clippy::pedantic)]

#[derive(Debug)]
pub enum LList<T: Copy> {
    Empty,
    Node(Box<(LList<T>, T)>),
}

impl<T: Copy> Default for LList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> LList<T> {
    pub fn new() -> Self {
        LList::Empty
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Node(boxed) => 1 + boxed.0.len(),
            Self::Empty => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn get(&self, i: usize) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::Node(rc) => {
                if i == 0 {
                    Some(rc.1)
                } else {
                    rc.0.get(i - 1)
                }
            }
        }
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = std::mem::replace(self, LList::Empty);
        *self = Self::Node(Box::new((new_node, value)));
    }

    pub fn delete(&mut self, i: usize) {
        if i == 0 {
            if let Self::Node(b) = std::mem::replace(self, LList::Empty) {
                *self = b.0;
            }
        } else if let Self::Node(ref mut b) = *self {
            b.0.delete(i - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_empty() {
        let mut list = LList::<i32>::new();
        assert_eq!(list.len(), 0);
        list.delete(13);
        list.delete(0);
        assert_eq!(list.get(0), None);
    }

    #[test]
    fn prepend_values() {
        let mut list = LList::new();
        list.prepend(10);
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.len(), 1);
        list.prepend(20);
        assert_eq!(list.get(0), Some(20));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn random_access() {
        let mut list = LList::new();
        for i in 0..5 {
            list.prepend(i);
        }
        assert_eq!(list.get(0), Some(4));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.get(2), Some(2));
        assert_eq!(list.get(3), Some(1));
        assert_eq!(list.get(4), Some(0));
        assert_eq!(list.get(5), None);
        assert_eq!(list.get(6), None);
        assert_eq!(list.len(), 5);

        list.delete(2);
        assert_eq!(list.len(), 4);
        assert_eq!(list.get(0), Some(4));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.get(2), Some(1));
        assert_eq!(list.get(3), Some(0));
    }
}
