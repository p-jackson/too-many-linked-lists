use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Default)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _phantom: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

#[must_use]
pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _phantom: PhantomData<&'a T>,
}

#[must_use]
pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _phantom: PhantomData<&'a mut T>,
}

#[must_use]
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_front = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            match self.front {
                Some(old_front) => {
                    (*old_front.as_ptr()).front = Some(new_front);
                    (*new_front.as_ptr()).back = Some(old_front);
                }
                None => {
                    self.back = Some(new_front);
                }
            }

            self.front = Some(new_front);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new_back = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            match self.back {
                Some(old_back) => {
                    (*old_back.as_ptr()).back = Some(new_back);
                    (*new_back.as_ptr()).front = Some(old_back);
                }
                None => {
                    self.front = Some(new_back);
                }
            }

            self.back = Some(new_back);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let owned_node = Box::from_raw(node.as_ptr());
                let result = owned_node.elem;

                self.front = owned_node.back;
                match self.front {
                    Some(node) => {
                        (*node.as_ptr()).front = None;
                    }
                    None => {
                        self.back = None;
                    }
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let owned_node = Box::from_raw(node.as_ptr());
                let result = owned_node.elem;

                self.back = owned_node.front;
                match self.back {
                    Some(node) => {
                        (*node.as_ptr()).back = None;
                    }
                    None => {
                        self.front = None;
                    }
                }

                self.len -= 1;
                result
            })
        }
    }

    #[must_use]
    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    #[must_use]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    #[must_use]
    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    #[must_use]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub const fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _phantom: PhantomData,
        }
    }

    pub const fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _phantom: PhantomData,
        }
    }

    pub const fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut {
            curr: None,
            list: self,
            index: None,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();
        new_list.extend(iter);
        new_list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

#[must_use]
pub struct CursorMut<'a, T> {
    curr: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

impl<T> CursorMut<'_, T> {
    #[must_use]
    pub const fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_next(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).back;
            }
            if self.curr.is_some() {
                *self.index.as_mut().unwrap() += 1;
            } else {
                self.index = None;
            }
        } else if !self.list.is_empty() {
            self.curr = self.list.front;
            self.index = Some(0);
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(curr) = self.curr {
            unsafe {
                self.curr = (*curr.as_ptr()).front;
            }
            if self.curr.is_some() {
                self.index.as_mut().map(|n| *n -= 1);
            } else {
                self.index = None;
            }
        } else if !self.list.is_empty() {
            self.curr = self.list.back;
            self.index = Some(self.list.len - 1);
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.curr.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(curr) = self.curr {
                // Normal case, try to follow the curr node's back pointer
                (*curr.as_ptr()).back
            } else {
                // Ghost case, try to use the list's front pointer
                self.list.front
            };

            // Yield the element if the next node exists
            next.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(curr) = self.curr {
                // Normal case, try to follow the curr node's front pointer
                (*curr.as_ptr()).front
            } else {
                // Ghost case, try to use the list's back pointer
                self.list.back
            };

            // Yield the element if the prev node exists
            prev.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(curr) = self.curr {
            unsafe {
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let prev = (*curr.as_ptr()).front;

                let new_len = old_len - old_index;
                let new_front = self.curr;
                let new_back = self.list.back;
                let new_index = Some(0);

                let out_len = old_index;
                let out_front = self.list.front;
                let out_back = prev;

                if let Some(prev) = prev {
                    (*curr.as_ptr()).front = None;
                    (*prev.as_ptr()).back = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_index;

                LinkedList {
                    front: out_front,
                    back: out_back,
                    len: out_len,
                    _phantom: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(curr) = self.curr {
            unsafe {
                let old_len = self.list.len;
                let old_index = self.index.unwrap();
                let next = (*curr.as_ptr()).back;

                let new_len = old_index + 1;
                let new_front = self.list.front;
                let new_back = self.curr;
                let new_index = Some(old_index);

                let out_len = old_len - new_len;
                let out_front = next;
                let out_back = self.list.back;

                if let Some(next) = next {
                    (*curr.as_ptr()).back = None;
                    (*next.as_ptr()).front = None;
                }

                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_index;

                LinkedList {
                    front: out_front,
                    back: out_back,
                    len: out_len,
                    _phantom: PhantomData,
                }
            }
        } else {
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        if input.is_empty() {
            return;
        }

        if let Some(curr) = self.curr {
            unsafe {
                let input_front = input.front.take().unwrap();
                let input_back = input.back.take().unwrap();

                if let Some(prev) = (*curr.as_ptr()).front {
                    (*prev.as_ptr()).back = Some(input_front);
                    (*input_front.as_ptr()).front = Some(prev);
                    (*curr.as_ptr()).front = Some(input_back);
                    (*input_back.as_ptr()).back = Some(curr);
                } else {
                    (*curr.as_ptr()).front = Some(input_back);
                    (*input_back.as_ptr()).back = Some(curr);
                    self.list.front = Some(input_front);
                }
                *self.index.as_mut().unwrap() += input.len;
            }
        } else if let Some(back) = self.list.back {
            unsafe {
                let input_front = input.front.take().unwrap();
                let input_back = input.back.take().unwrap();

                (*back.as_ptr()).back = Some(input_front);
                (*input_front.as_ptr()).front = Some(back);
                self.list.back = Some(input_back);
            }
        } else {
            std::mem::swap(self.list, &mut input);
        }

        self.list.len += input.len;
        input.len = 0;
    }

    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        if input.is_empty() {
            return;
        }

        if let Some(curr) = self.curr {
            unsafe {
                let input_front = input.front.take().unwrap();
                let input_back = input.back.take().unwrap();

                if let Some(next) = (*curr.as_ptr()).back {
                    (*next.as_ptr()).front = Some(input_back);
                    (*input_back.as_ptr()).back = Some(next);
                    (*curr.as_ptr()).back = Some(input_front);
                    (*input_front.as_ptr()).front = Some(curr);
                } else {
                    (*curr.as_ptr()).back = Some(input_front);
                    (*input_front.as_ptr()).front = Some(curr);
                    self.list.back = Some(input_back);
                }
                *self.index.as_mut().unwrap() += input.len;
            }
        } else if let Some(front) = self.list.front {
            unsafe {
                let input_front = input.front.take().unwrap();
                let input_back = input.back.take().unwrap();

                (*front.as_ptr()).front = Some(input_back);
                (*input_back.as_ptr()).back = Some(front);
                self.list.front = Some(input_front);
            }
        } else {
            std::mem::swap(self.list, &mut input);
        }

        self.list.len += input.len;
        input.len = 0;
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}
unsafe impl<T: Send> Send for Iter<'_, T> {}
unsafe impl<T: Sync> Sync for Iter<'_, T> {}
unsafe impl<T: Send> Send for IterMut<'_, T> {}
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}

#[allow(dead_code)]
const fn assert_properties() {
    const fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> {
        x
    }
    const fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> {
        x
    }
    const fn into_iter_covariant<'a, T>(x: IntoIter<&'static T>) -> IntoIter<&'a T> {
        x
    }

    /// ```compile_fail,E0308
    /// use linked_list::IterMut;
    ///
    /// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>) -> IterMut<'i, &'a T> { x }
    /// ```
    const fn iter_mut_invariant() {}

    const fn is_send<T: Send>() {}
    const fn is_sync<T: Sync>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();

    is_send::<IntoIter<i32>>();
    is_sync::<IntoIter<i32>>();

    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    is_send::<IterMut<i32>>();
    is_sync::<IterMut<i32>>();
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let as_i32 = i as i32;
            assert_eq!(as_i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let as_i32 = i as i32;
            assert_eq!(6 - as_i32, *elt);
        }
        let mut n = LinkedList::new();
        #[allow(clippy::manual_next_back)]
        let next_back = n.iter().rev().next();
        assert_eq!(next_back, None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let as_i32 = i as i32;
            assert_eq!(as_i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        use std::cmp::Ordering as O;
        let nan = f64::NAN;
        let has_nan = list_from(&[nan]);
        let has_nan_2 = list_from(&[nan]);
        assert_ne!(Some(O::Less), has_nan.partial_cmp(&has_nan_2));
        assert_ne!(Some(O::Greater), has_nan.partial_cmp(&has_nan_2));
        assert_ne!(Some(O::Equal), has_nan.partial_cmp(&has_nan_2));

        let has_nan = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert_ne!(Some(O::Less), has_nan.partial_cmp(&one));
        assert_ne!(Some(O::Greater), has_nan.partial_cmp(&one));
        assert_ne!(Some(O::Equal), has_nan.partial_cmp(&one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert_ne!(Some(O::Less), u.partial_cmp(&v));
        assert_ne!(Some(O::Greater), u.partial_cmp(&v));
        assert_ne!(Some(O::Equal), u.partial_cmp(&v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert_ne!(Some(O::Less), s.partial_cmp(&t));
        assert_ne!(Some(O::Equal), s.partial_cmp(&t));
        assert_eq!(Some(O::Greater), s.partial_cmp(&one));
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{list:?}"), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = ["just", "one", "test", "more"].into_iter().collect();
        assert_eq!(format!("{list:?}"), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }

    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));

        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }

    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.splice_before(Some(7).into_iter().collect());
        cursor.splice_after(Some(8).into_iter().collect());
        // check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[7, 1, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
        );

        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 8, 2, 3, 4, 5, 6]);

        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        let tmp = cursor.split_after();
        assert_eq!(
            tmp.into_iter().collect::<Vec<_>>(),
            &[102, 103, 8, 2, 3, 4, 5, 6]
        );
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101]
        );
    }

    fn check_links<T: std::fmt::Debug + PartialEq>(list: &LinkedList<T>) {
        let from_front: Vec<_> = list.iter().collect();
        let from_back: Vec<_> = list.iter().rev().collect();
        let re_reved: Vec<_> = from_back.into_iter().rev().collect();

        assert_eq!(from_front, re_reved);
    }
}
