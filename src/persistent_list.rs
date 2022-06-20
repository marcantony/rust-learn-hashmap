use std::rc::Rc;

pub mod iter;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>
}

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn prepend(&self, item: T) -> Self {
        let new_node = Rc::new(Node {
            item: item,
            next: self.head.clone()
        });

        LinkedList { head: Some(new_node) }
    }

    pub fn tail(&self) -> Self {
        LinkedList { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(link) = head {
            if let Ok(node) = Rc::try_unwrap(link) {
                // If this list is the only reference to the node, take ownership of
                // it and subsequently drop it
                head = node.next;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let list = LinkedList::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}
