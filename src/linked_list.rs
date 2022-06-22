pub mod into_iter;
pub mod iter;
pub mod iter_mut;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>
}

/// A mutable, stack-like linked list.
pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    /// Pushes an item to the head of the list.
    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    fn pop_link(&mut self) -> Link<T> {
        self.head.take().map(|mut boxed_node| {
            self.head = boxed_node.next.take();
            boxed_node
        })
    }

    /// Removes the list's head and returns it, if it exists.
    pub fn pop(&mut self) -> Option<T> {
        self.pop_link().map(|node| node.item)
    }

    /// Returns a shared reference to the list's head, if it exists.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    /// Returns a mutable reference to the lists's head, if it exists.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }
}

// https://rust-unofficial.github.io/too-many-lists/first-drop.html
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_link() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::<i32>::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::<i32>::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
    }

    #[test]
    fn test_peek_mutability() {
        let mut list = LinkedList::<i32>::new();

        list.push(1);

        list.peek_mut()
            .map(|value| *value = 2);
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.pop();
        assert_eq!(list.peek_mut(), None);
    }
}
